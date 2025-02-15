import requests
import json
import time
from urllib.parse import urljoin
import sys

def test_jupyter_server(base_url="http://localhost:7999"):
    """
    Test Jupyter server connectivity and basic functionality
    Returns (success: bool, message: str)
    """
    try:
        # Step 0: Clean up any existing kernels
        print("Cleaning up existing kernels...")
        sessions_url = urljoin(base_url, "/api/sessions")
        response = requests.get(sessions_url, timeout=10)
        if response.status_code == 200:
            existing_sessions = response.json()
            for session in existing_sessions:
                session_id = session.get('id')
                if session_id:
                    requests.delete(f"{sessions_url}/{session_id}")
            print(f"Cleaned up {len(existing_sessions)} existing sessions")
        
        # Step 1: Test basic connectivity and get XSRF token
        print("Testing basic connectivity...")
        response = requests.get(base_url + "/lab", 
                              headers={"User-Agent": "Mozilla/5.0"},
                              timeout=10)
        response.raise_for_status()
        
        # Extract XSRF token from cookies
        xsrf_token = response.cookies.get('_xsrf')
        if not xsrf_token:
            return False, "Failed to get XSRF token"
        print(f"Got XSRF token: {xsrf_token}")

        # Step 2: Create a new session
        print("Creating new session...")
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
        print(f"Created session: {json.dumps(session_info, indent=2)}")

        # Step 3: Test kernel by executing code through session
        print("Testing kernel execution...")
        execute_url = urljoin(base_url, f"/api/sessions/{session_info['id']}/execute")
        code = "print('Hello from Jupyter kernel')"
        
        for attempt in range(max_retries):
            try:
                # Try to execute code via session
                response = requests.post(execute_url,
                                      headers=headers,
                                      json={
                                          "code": code,
                                          "silent": False,
                                          "store_history": True,
                                          "user_expressions": {},
                                          "allow_stdin": False
                                      },
                                      timeout=10)
                response.raise_for_status()
                print(f"Response from execute: {json.dumps(response.json(), indent=2)}")
                return True, "Successfully sent code to kernel"
                    
            except requests.exceptions.RequestException as e:
                print(f"Attempt {attempt + 1} failed: {str(e)}")
                if attempt < max_retries - 1:
                    print(f"Waiting {retry_delay}s before retry...")
                    time.sleep(retry_delay)
                    
        return False, "Failed to execute code after multiple attempts"

        # Step 4: Clean up
        print("Cleaning up...")
        response = requests.delete(kernel_url,
                                 headers=headers,
                                 timeout=10)
        response.raise_for_status()
        
        return True, "All tests passed successfully"

    except requests.exceptions.ConnectionError as e:
        return False, f"Connection error: {str(e)}"
    except requests.exceptions.Timeout as e:
        return False, f"Timeout error: {str(e)}"
    except requests.exceptions.RequestException as e:
        return False, f"Request error: {str(e)}"
    except Exception as e:
        return False, f"Unexpected error: {str(e)}"

if __name__ == "__main__":
    import os
    # Get port from environment if provided, otherwise use default
    port = os.environ.get('RBN_ENTRY_PORT_WORKSTATION', '7999')
    base_url = f"http://localhost:{port}"
    success, message = test_jupyter_server(base_url)
    print(f"\nTest result: {message}")
    sys.exit(0 if success else 1)

