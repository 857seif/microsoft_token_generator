from mitmproxy import http
import json
def response(flow: http.HTTPFlow):
    if flow.request.pretty_url == 'https://licensing.mp.microsoft.com/v8.0/licenseToken':
        flow.response.headers['content-type'] = 'application/json'
        flow.response.text = json.dumps({'licenseToken': ''})