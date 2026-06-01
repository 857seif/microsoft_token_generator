import json
import logging
import os
from mitmproxy import http


logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")


TRIGGER_FILE = r"C:\Users\10\AppData\Local\Temp\.tmpZPmhUf\patched.signal"

MICROSOFT_LICENSE_TOKEN = (
    "eyJhbGciOiJSUzI1NiIsImtpZCI6IjJGOURBRkE1RURFMkIwNEU0QTRGOEQ1QzVBRUQyOTE1NzE3"
    "MUIzMDMiLCJ4NXQiOiJMNTJ2cGUzaXNFNUtUNDFjV3UwcEZYRnhzd00iLCJ0eXAiOiJKV1QifQ."
    "eyJMaWNlbnNlVG9rZW5DbGFpbSI6ImV5SmpaWEowYVdacFkyRjBaVWxrSWpvaU1rWTVSRUZHUVR"
    "WRlJFVXlRakEwUlRSQk5FWTRSRFZETlVGRlJESTVNVFUzTVRjeFFqTXdNeUlzSW1OMWMzUnZiVV"
    "JsZG1Wc2IzQmxjbE4wY21sdVp5STZJakkxTXpVME5qWTVNVGt3TmpVNU56QWlMQ0pzYVdObGJu"
    "TmhZbXhsVUhKdlpIVmpkSE1pT2x0N0ltVnVaRVJoZEdVaU9pSTVPVGs1TFRFeUxUTXhWREl6T2"
    "pVNU9qVTVMams1T1RrNU9Ua3JNREE2TURBaUxDSnBjMU5vWVhKbFpDSTZabUZzYzJVc0ltbGtk"
    "R1YyU21kSWNXaDBOa1J2UjBSTVkySklhVzFEV0VaQ2EyRmFXbE5RTTA5amRqVnZXVU5FWVVaNE"
    "JrU0VoU01uUTRiVk50WVVoVUsxWnNWM1YwWjJNNFBTSjlMSHNpWlc1a1JHRjBaU0k2SWprNU9U"
    "a3RNVEl0TXpGVU1qTTZOVGs2TlRrdU9UazVPVGs1T1Nzd01Eb3dNQ0lzSW1selUyaGhjbVZrSW"
    "pwMGNuVmxMQ0pwWkNJNkltTTJPVEEzTm1ReU1HVTBaVFEzTjJKaU5qaGpZV0ZqT0RWaVlUQTRZ"
    "ekEzSWl3aWNISnZaSFZqZEVsa0lqb2lPVTVFUmpGR01qWXpVbG8wSWl3aWMydDFTV1FpT2lJd0"
    "1ERXdJaXdpZFhObGNrbGtJam9pWm1OTmJtNXhXR0VyV0RkRlZEa3libkIxUVZWM1pHVkZWeko2"
    "ZnpRMk1sVlJhbXRHYkhjM1luWktZejBpZlYwc0luQmhlV3h2WVdRaU9pSlhTR1ZDUW0wNGVHM"
    "WhNeTkxVTFsUWVXMUROMXBLUVRjNWRtNVdha1pWUW1Wd1MyZzViSEo2TUhCT1EyZG5kRlJyYmp"
    "OVlpXa3lhbXcxUkZjclJVUlRhVXRIVVdOTlUxaFJZMVZVYzJwSFFWSk1SM0oxVUdFMWRVNHlP"
    "VEF4ZWtSd0wzQnVWMGxHWTIxRVJWTmpheTh2WTNGUU1FeEJTR3hsTHpKd2IyeFFLMFZqZGtGY"
    "VdXSjNjVzlQTWxCSk9YVm9RMFZOVjNkUlkzcHRkVlkwYms1cmNYTXZPVUV2VUZOMWRHOUxZbU"
    "Z4UkdaWFNFRTRhemRNU2s5cGNWcDVUMUpDWW1KemMwZE5NR2hGUkdJME9XMHZXRVJJYW04Mk1"
    "UWTVWa1k1T0ZSYWVEbGtkR0YyU21kSWNXaDBOa1J2UjBSTVkySklhVzFEV0VaQ2EyRmFXbE5R"
    "TTA5amRqVnZXVU5FWVVaNFNrOUVhelZYWjFadVVtTXpTVFpLTjNaNFVsSXZaM1JGWnpOYWEyc"
    "GhkVWN4TWtjeFdGRldka1JVVDNCV2JWSlFNRVpKTmxoamJGZGxUamxUUjNZek1HeHFOak51T0"
    "dkUlUzYzlQU0lzSW5SdmEyVnVWbVZ5YzJsdmJpSTZNWDA9IiwibmJmIjoxNzc5Nzg1MjI4LCJ"
    "leHAiOjE3N9A4NzE2MjgsImlzcyI6IkxpY2Vuc2luZ0ZEIiwiYXVkIjoiU0RLL0lTViJ9.Oyx"
    "zFRMMBvj7LzocqSPDD5gHpEu-c3QGaeN0AGt9bodwijNLXdnQgHS4ruGrLTndYcQSm25dkKe5"
    "xEXcLtHbJWKBnvPyi8Lwr2-fQv7b5H8Lhl8PGtnGO7LjM1OiEtpy0es89eS3MI9Yny9lnxBX"
    "VSe0B1g9FNAsJMtPfevZI2t12UcA_KXNySBttm4wpRYTbGeMXTP4vZJ3pxgmq3wOtOBatwKs"
    "7z3lquBAurCPWvT9VuYWUN_XSMyiOBQrp_hzSQBeYYDE9ow8pUCR7RMWg66mY1kCJcBTJnD8"
    "ImikHrsv0RIpF1sAvVHiwYHbO1yB2YPLP5RLMJwjTaQcw5ucTg"
)

def request(flow: http.HTTPFlow):

    target_host = "licensing.mp.microsoft.com"
    target_path = "/v8.0/licenseToken"

    if target_host in flow.request.pretty_host and target_path in flow.request.path:
        logging.info(f"[+] Intercepted Microsoft License Request from: {flow.client_conn.peername[0]}")
        
       
        response_data = {"licenseToken": MICROSOFT_LICENSE_TOKEN}
        flow.response = http.Response.make(
            200,                                        
            json.dumps(response_data),                  
            {"Content-Type": "application/json"}       
        )
        logging.info("[+] Fake License Token injected successfully!")
        
        
        create_trigger_file()

def create_trigger_file():
    
    try:
        folder_path = os.path.dirname(TRIGGER_FILE)
        if not os.path.exists(folder_path):
            os.makedirs(folder_path, exist_ok=True)
            
        with open(TRIGGER_FILE, "w") as f:
            f.write("patched")
        logging.info(f"[+] Signal file created at: {TRIGGER_FILE}")
    except IOError as e:
        logging.error(f"[-] Failed to write signal file: {e}")