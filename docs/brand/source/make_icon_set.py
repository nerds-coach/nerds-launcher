"""Writes a NERDS replacement for every icon file an upstream Craft app has in assets/app-icon/,
with the same name, format and size. usage: make_icon_set.py <app-svg> <upstream assets/app-icon> <out>"""
import io, sys, re, shutil, pathlib, subprocess
from PIL import Image
from playwright.sync_api import sync_playwright

svg_path, up, out = map(pathlib.Path, sys.argv[1:4])
FLAT = svg_path.read_text()
# macOS: the tile on Apple's 824/1024 grid (transparent margin round it).
MAC = FLAT.replace('viewBox="0 0 1024 1024"', 'viewBox="-124.25 -124.25 1272.5 1272.5"')
assert MAC != FLAT
SKIP = re.compile(r'(README|LICENSE|\.attribution$|icon-source|-document\.)')

page = None
def render(svg, size):
    page.set_viewport_size({'width': size, 'height': size})
    page.set_content('<html><body style="margin:0;background:transparent">'
                     + svg.replace('<svg ', f'<svg width="{size}" height="{size}" ', 1) + '</body></html>')
    return Image.open(io.BytesIO(page.screenshot(omit_background=True, clip={'x': 0, 'y': 0, 'width': size, 'height': size}))).convert('RGBA')

done = []
with sync_playwright() as p:
    browser = p.chromium.launch(); page = browser.new_page()
    for f in sorted(up.rglob('*')):
        if not f.is_file(): continue
        rel = f.relative_to(up)
        if SKIP.search(f.name): continue
        dest = out / rel; dest.parent.mkdir(parents=True, exist_ok=True)
        ext = f.suffix.lower()
        mac = ('-1024' in f.name) or ('macos' in f.name)
        if ext == '.svg':
            dest.write_text(FLAT)
        elif ext == '.png':
            size = Image.open(f).size
            im = render(MAC if mac else FLAT, size[0])
            if size[1] != size[0]: im = im.resize(size)
            im.save(dest)
        elif ext == '.ico':
            sizes = sorted(Image.open(f).info.get('sizes', {(16, 16), (32, 32), (48, 48), (256, 256)}))
            ims = [render(FLAT, s[0]) for s in sizes]
            ims[-1].save(dest, format='ICO', sizes=sizes, append_images=ims[:-1])
        elif ext == '.icns':
            render(MAC, 1024).save(dest, format='ICNS')
        else:
            continue
        done.append(str(rel))
    browser.close()
print(len(done), 'files:', ' '.join(done))
