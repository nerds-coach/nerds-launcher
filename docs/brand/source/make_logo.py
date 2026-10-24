"""Build NERDS logo SVGs: the N mark constructed geometrically from measurements of the
supplied 2677x2825 PNG, and the N.E.R.D.S. wordmark traced from it."""
import math, sys
import numpy as np, cv2
from PIL import Image

SRC = '/mnt/user-data/uploads/UPDATED-STACK_transparent.png'
BLUE = '#0077C0'
k = 1.18          # slope of every slanted cut and of the diagonal
tv = 216.0        # vertical thickness of the diagonal band

def cut(x0, y0):  # y on a slanted line through (x0, y0)
    return lambda x: y0 + k * (x - x0)

# Left U: legs A (537-677) and C (1076-1216), arc centre (876.5, 1679.5)
A0, A1, C0, C1 = 537, 677, 1076, 1216
cxL, cyL, R, r = 876.5, 1679.5, 339.5, 199.5
a_top = cut(539, 100); c_top = cut(1078, 1088)
left_u = (f"M{A0},{a_top(A0):.1f} L{A1},{a_top(A1):.1f} L{A1},{cyL} "
          f"A{r},{r} 0 0 0 {C0},{cyL} L{C0},{c_top(C0):.1f} L{C1},{c_top(C1):.1f} L{C1},{cyL} "
          f"A{R},{R} 0 0 1 {A0},{cyL} Z")

# Middle bar B (806-946) + diagonal + bar E (1732-1872)
B0, B1, E0, E1 = 806, 946, 1732, 1872
up = cut(806, 416.6); lo = lambda x: up(x) + tv
Etop, Bbot = 602.5, 1679.5
diag = (f"M{B0},{Bbot} L{B0},{up(B0):.1f} L{E0},{up(E0):.1f} L{E0},{Etop} L{E1},{Etop} "
        f"L{E1},{lo(E1):.1f} L{B1},{lo(B1):.1f} L{B1},{Bbot} Z")

# Right U: legs D (1463-1603) and F (2001-2141), arc centre (1802, 602.5)
D0, D1, F0, F1 = 1462.5, 1602.5, 2001.5, 2141.5
cxR, cyR = 1802, 602.5
d_bot = cut(1465, 1062); f_bot = cut(2002, 2047)
right_u = (f"M{D0},{d_bot(D0):.1f} L{D0},{cyR} A{R},{R} 0 0 1 {F1},{cyR} L{F1},{f_bot(F1):.1f} "
           f"L{F0},{f_bot(F0):.1f} L{F0},{cyR} A{r},{r} 0 0 0 {D1},{cyR} L{D1},{d_bot(D1):.1f} Z")

MARK = [left_u, diag, right_u]

# Wordmark: trace the text band of the PNG.
im = np.array(Image.open(SRC).convert('RGBA'))
alpha = im[:, :, 3]
text = np.zeros_like(alpha); text[2260:2700] = alpha[2260:2700]
_, bw = cv2.threshold(text, 127, 255, cv2.THRESH_BINARY)
contours, hier = cv2.findContours(bw, cv2.RETR_CCOMP, cv2.CHAIN_APPROX_NONE)
word = []
for c in contours:
    c = cv2.approxPolyDP(c, 0.9, True).reshape(-1, 2)
    if len(c) < 3: continue
    word.append('M' + ' L'.join(f'{x},{y}' for x, y in c) + ' Z')
WORD = ' '.join(word)
ys, xs = np.where(bw > 0)
wb = (xs.min(), ys.min(), xs.max() + 1, ys.max() + 1)

def svg(paths, box, fill, pad=0):
    x0, y0, x1, y1 = box
    vb = f'{x0 - pad:.1f} {y0 - pad:.1f} {x1 - x0 + 2 * pad:.1f} {y1 - y0 + 2 * pad:.1f}'
    body = ''.join(f'<path d="{p}"/>' for p in paths)
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}">'
            f'<g fill="{fill}" fill-rule="evenodd">{body}</g></svg>\n')

mark_box = (537, 97, 2142, 2212)
stack_box = (min(537, wb[0]), 97, max(2142, wb[2]), wb[3])
out = sys.argv[1]
for name, fill in [('', BLUE), ('-white', '#FFFFFF'), ('-black', '#000000')]:
    open(f'{out}/nerds-mark{name}.svg', 'w').write(svg(MARK, mark_box, fill))
    open(f'{out}/nerds-logo-stacked{name}.svg', 'w').write(svg(MARK + [WORD], stack_box, fill))
    open(f'{out}/nerds-wordmark{name}.svg', 'w').write(svg([WORD], wb, fill))
print('wordmark box', wb, 'contours', len(word))

# Horizontal lockup for headers: mark, then the wordmark at 38% of the mark's height, centred.
mw, mh = 2142 - 537, 2212 - 97
ww, wh = wb[2] - wb[0], wb[3] - wb[1]
s = 0.38 * mh / wh
gap = 0.30 * mw
W = mw + gap + ww * s
for name, fill in [('', BLUE), ('-white', '#FFFFFF'), ('-black', '#000000'), ('-ink', '#0B1520')]:
    mark = ''.join(f'<path d="{p}"/>' for p in MARK)
    body = (f'<g transform="translate({-537},{-97})">{mark}</g>'
            f'<g transform="translate({mw + gap:.1f},{(mh - wh * s) / 2:.1f}) scale({s:.4f}) translate({-wb[0]},{-wb[1]})"><path d="{WORD}"/></g>')
    open(f'{out}/nerds-logo-horizontal{name}.svg', 'w').write(
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W:.1f} {mh}" role="img" aria-label="NERDS">'
        f'<title>NERDS</title><g fill="{fill}" fill-rule="evenodd">{body}</g></svg>\n')
print('horizontal aspect', W / mh)
