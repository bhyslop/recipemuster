import requests
import json
import time
from urllib.parse import urljoin
import sys
import uuid
import websocket
import threading

def test_jupyter_server(base_url="http://localhost:7999"):
    """
    Test Jupyter server connectivity and basic functionality
    Returns (success: bool, message: str)
    """
    try:
        # Step 0: Clean up any existing kernels
        sessions_url = urljoin(base_url, "/api/sessions")
        response = requests.get(sessions_url, timeout=10)
        if response.status_code == 200:
            existing_sessions = response.json()
            for session in existing_sessions:
                session_id = session.get('id')
                if session_id:
                    requests.delete(f"{sessions_url}/{session_id}")
        
        # Step 1: Test basic connectivity and get XSRF token
        response = requests.get(base_url + "/lab", 
                              headers={"User-Agent": "Mozilla/5.0"},
                              timeout=10)
        response.raise_for_status()
        
        xsrf_token = response.cookies.get('_xsrf')
        if not xsrf_token:
            return False, "Failed to get XSRF token"

        # Step 2: Create a new session
        api_url = urljoin(base_url, "/api/sessions")
        headers = {
            "Content-Type": "application/json",
            "X-XSRFToken": xsrf_token,
            "Cookie": f"_xsrf={xsrf_token}"
        }
        
        session_data = {
            "kernel": {"name": "python3"},
            "name": "test.ipynb",
            "path": "test.ipynb",
            "type": "notebook"
        }
        
        response = requests.post(api_url, 
                               headers=headers,
                               json=session_data,
                               timeout=10)
        response.raise_for_status()
        session_info = response.json()
        if not session_info.get('id'):
            return False, "Failed to create session"

        # Step 3: Test kernel by executing code through WebSocket
        kernel_id = session_info['kernel']['id']
        ws_url = urljoin(base_url, f"/api/kernels/{kernel_id}/channels")
        ws_url = ws_url.replace('http://', 'ws://')
        
        kernel_ready = threading.Event()
        execution_completed = threading.Event()
        execution_successful = False
        execution_result = None
        
        def on_message(ws, message):
            nonlocal execution_successful, execution_result
            try:
                msg = json.loads(message)
                msg_type = msg.get('msg_type', '')
                parent_header = msg.get('parent_header', {})
                
                if msg_type == 'status':
                    state = msg.get('content', {}).get('execution_state')
                    if state == 'idle':
                        if parent_header.get('msg_type') == 'kernel_info_request':
                            kernel_ready.set()
                        else:
                            execution_completed.set()
                elif msg_type == 'stream':
                    text = msg.get('content', {}).get('text', '')
                    execution_successful = True
                    execution_result = text
                elif msg_type == 'execute_result':
                    data = msg.get('content', {}).get('data', {})
                    execution_successful = True
                    execution_result = str(data.get('text/plain', ''))
                elif msg_type == 'error':
                    execution_completed.set()
            except Exception as e:
                print(f"Error processing message: {str(e)}")

        def on_error(ws, error):
            print(f"WebSocket error: {error}")

        def on_close(ws, close_status_code, close_msg):
            pass

        def on_open(ws):
            kernel_info_request = {
                'header': {
                    'msg_id': str(uuid.uuid4()),
                    'username': '',
                    'session': str(uuid.uuid4()),
                    'msg_type': 'kernel_info_request',
                    'version': '5.3'
                },
                'parent_header': {},
                'metadata': {},
                'content': {},
                'channel': 'shell',
                'buffers': []
            }
            ws.send(json.dumps(kernel_info_request))

        ws = websocket.WebSocketApp(
            ws_url,
            header=[
                f"Cookie: _xsrf={xsrf_token}",
                "User-Agent: Mozilla/5.0"
            ],
            on_open=on_open,
            on_message=on_message,
            on_error=on_error,
            on_close=on_close
        )

        ws_thread = threading.Thread(target=ws.run_forever, daemon=True)
        ws_thread.start()

        if not kernel_ready.wait(timeout=10):
            ws.close()
            return False, "Timeout waiting for kernel to be ready"

        execute_request = {
            'header': {
                'msg_id': str(uuid.uuid4()),
                'username': '',
                'session': str(uuid.uuid4()),
                'msg_type': 'execute_request',
                'version': '5.3'
            },
            'parent_header': {},
            'metadata': {},
            'content': {
                'code': 'print("Hello from Jupyter kernel")',
                'silent': False,
                'store_history': True,
                'user_expressions': {},
                'allow_stdin': False
            },
            'channel': 'shell',
            'buffers': []
        }
        ws.send(json.dumps(execute_request))

        if not execution_completed.wait(timeout=10):
            ws.close()
            return False, "Timeout waiting for execution result"

        ws.close()
        
        if execution_successful:
            return True, f"Successfully executed code. Result: {execution_result}"
        else:
            return False, "Failed to get execution result"

    except requests.exceptions.ConnectionError as e:
        if 'ws' in locals():
            ws.close()
        return False, f"Connection error: {str(e)}"
    except requests.exceptions.Timeout as e:
        if 'ws' in locals():
            ws.close()
        return False, f"Timeout error: {str(e)}"
    except requests.exceptions.RequestException as e:
        if 'ws' in locals():
            ws.close()
        return False, f"Request error: {str(e)}"
    except Exception as e:
        if 'ws' in locals():
            ws.close()
        return False, f"Unexpected error: {str(e)}"

if __name__ == "__main__":
    import os
    port = os.environ.get('RBN_ENTRY_PORT_WORKSTATION', '7999')
    base_url = f"http://localhost:{port}"
    success, message = test_jupyter_server(base_url)
    print(f"Test result: {message}")
    sys.exit(0 if success else 1)

