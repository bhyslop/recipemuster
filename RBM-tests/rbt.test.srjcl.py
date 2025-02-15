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
        kernel_id = session_info.get('kernel', {}).get('id')
        if not kernel_id:
            return False, "Failed to create kernel session"
        print(f"Created kernel session: {kernel_id}")

        # Step 3: Test kernel status
        print("Testing kernel status...")
        kernel_url = urljoin(base_url, f"/api/kernels/{kernel_id}")
        response = requests.get(kernel_url,
                              headers=headers,
                              timeout=10)
        response.raise_for_status()
        kernel_info = response.json()
        if kernel_info.get('status') != 'idle':
            return False, f"Unexpected kernel status: {kernel_info.get('status')}"
        print("Kernel is running and idle")

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

