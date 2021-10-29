#!/usr/bin/env python3

"""
curl -O https://www.iana.org/assignments/media-types/media-types.txt
"""


def write_mime(content: str, w, keys: set):
    content = content.strip()
    if len(content) == 0:
        return

    k ,v = None, None
    terms = content.split(' ')
    for term in terms:
        term = term.strip()
        if len(term) == 0:
            continue
        if not k:
            k = term
            continue
        if not v:
            v = term
    if not k or not v or '-' in k or '+' in k or '.' in k or not ('/' in v) or len(k) > 5:
        return
    k = k.lower()
    if k in keys:
        return
    keys.add(k)
    output = "%s=%s\n" % (k ,v.lower())
    w.write(output)


def main():
    keys = set()
    with open('media-types.properties', 'w+') as w:
        with open('media-types.txt', 'r') as r:
            lines = r.readlines()
            for line in lines:
                write_mime(line, w, keys)


if __name__ == '__main__':
    main()
