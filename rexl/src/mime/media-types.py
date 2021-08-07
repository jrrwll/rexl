#!/usr/bin/env python3

"""
curl -O https://www.iana.org/assignments/media-types/media-types.txt
"""


def write_mime(content: str, w):
    if content[0:1] in ' \t\b' or len(content.strip()) == 0:
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
    if not k or not v or '-' in k or '+' in k or '.' in k or not ('/' in v):
        return
    output = "%s=%s\n" % (k.lower() ,v)
    print(output)
    w.write(output)


def main():
    with open("./media-types.properties", 'w+') as w:
        with open('./media-types.txt', 'r') as r:
            lines = r.readlines()
            for line in lines:
                write_mime(line, w)


if __name__ == '__main__':
    main()
