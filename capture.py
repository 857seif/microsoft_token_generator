import json
from mitmproxy import http
def response(flow: http.HTTPFlow):
    if flow.response and 'licensing.mp.microsoft.com' in flow.request.pretty_host and '/v8.0/licenseToken' in flow.request.path:
        if flow.response.status_code == 200 and flow.response.content:
            try:
                data = json.loads(flow.response.text)
                token = data.get('licenseToken')
                if token:
                    with open(r'C:\\Users\\SKY TOP\\Desktop/captured_token.txt', 'w', encoding='utf-8') as f: f.write(token)
            except: pass