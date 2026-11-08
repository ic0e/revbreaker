import base64

TRANSFORMS = {
    "base32_dec": ("decode", lambda d: base64.b32decode(d)),
    "base32_enc": ("encode", lambda d: base64.b32encode(d)),
}


def transform(data):
    out = []
    for name, (kind, fn) in TRANSFORMS.items():
        try:
            res = fn(data)
        except Exception:
            continue
        if res and res != data:
            out.append((name, res))
    return out
