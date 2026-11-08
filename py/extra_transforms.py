import sys
import base64
import json

TRANSFORMS = {
    "base32_dec": ("decode", lambda d: base64.b32decode(d)),
    "base32_enc": ("encode", lambda d: base64.b32encode(d)),
}


def main():
    req = json.load(sys.stdin)
    data = bytes.fromhex(req["input_hex"])
    # include_encoders = req.get("encode", False) # add later for optional encoding
    out = []

    for name, (kind, fn) in TRANSFORMS.items():
        try:
            res = fn(data)
        except Exception:
            continue
        if res and res != data:
            out.append({"name": name, "kind": kind, "output_hex": res.hex()})

    json.dump(out, sys.stdout)

if __name__ == "__main__":
    main()
