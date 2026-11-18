"""NERDS app icons: one tile per app in its own colour, with a symbol for what the app does.
All shapes are plain SVG paths (no fonts), drawn on a 1024 grid."""
import math, re, sys
from pathlib import Path

W = '@W@'              # symbol: white, or the app's own symbol colour (filled in per app)
B = '@B@'              # cut-outs inside the symbol: the tile colour (filled in per app)
L = '@L@'              # secondary details: a tint between the symbol and the tile (per app)

# Each app's colours, as NERDS chose them (2026-11-18): the tile, and the symbol when it isn't
# white (NERDS Photo is a navy tile with a light-blue symbol). The launcher's own tile is NERDS blue.
COLOURS = {
    'pdfcraft': '#f40f02', 'effectcraft': '#9999ff', 'soundcraft': '#9999ff', 'vectorcraft': '#ff9a00',
    'designcraft': '#ff3366', 'lightcraft': '#31a8ff', 'photocraft': ('#001e36', '#31a8ff'),
    'filmcraft': '#9999ff', 'cadcraft': '#e51050', 'figcraft': '#ff61f6', 'gridcraft': '#217346',
    'deckcraft': '#d04423', 'wordcraft': '#1b5ebe', 'nerds-launcher': '#0077c0',
}

def _rgb(h): return [int(h[i:i + 2], 16) for i in (1, 3, 5)]
def _hex(c): return '#%02x%02x%02x' % tuple(max(0, min(255, round(v))) for v in c)
def mix(h, other, t): return _hex([a + (b - a) * t for a, b in zip(_rgb(h), _rgb(other))])
def lighten(h, t): return mix(h, '#ffffff', t)
def darken(h, t): return mix(h, '#000000', t)

def colours(app):
    """(tile, symbol) for an app."""
    c = COLOURS[app]
    return c if isinstance(c, tuple) else (c, '#ffffff')

def palette(tile_colour, symbol):
    """Tile gradient (top, bottom), symbol, cut-out and secondary-detail colours. The tile's
    middle is the chosen colour itself; every symbol has a soft shadow, so white stays readable
    on the lighter tiles."""
    light = lighten(tile_colour, 0.58) if symbol == '#ffffff' else mix(symbol, tile_colour, 0.42)
    return lighten(tile_colour, 0.09), darken(tile_colour, 0.10), symbol, darken(tile_colour, 0.04), light

MARK = re.search(r'<g fill="[^"]+" fill-rule="evenodd">(.*?)</g>',
                 Path(sys.argv[1]).read_text()).group(1)   # nerds-mark.svg paths

def tile(body, name, app):
    top, bottom, symbol, cut, light = palette(*colours(app))
    body = body.replace('@W@', symbol).replace('@B@', cut).replace('@L@', light)
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" role="img" aria-label="{name}">'
            f'<title>{name}</title><defs><linearGradient id="g" x1="0" y1="0" x2="0" y2="1">'
            f'<stop offset="0" stop-color="{top}"/><stop offset="1" stop-color="{bottom}"/></linearGradient>'
            f'<filter id="s" x="-20%" y="-20%" width="140%" height="140%"><feDropShadow dx="0" dy="10" stdDeviation="14" '
            f'flood-color="#000000" flood-opacity="0.22"/></filter></defs>'
            f'<rect width="1024" height="1024" rx="230" fill="url(#g)"/><g filter="url(#s)">{body}</g></svg>\n')

def s(d, w=56, c=W, cap='round', join='round'):
    return f'<path d="{d}" fill="none" stroke="{c}" stroke-width="{w}" stroke-linecap="{cap}" stroke-linejoin="{join}"/>'

def f(d, c=W):
    return f'<path d="{d}" fill="{c}"/>'

def star(cx, cy, r, c=W):  # four-point sparkle
    k = 0.18 * r
    return f(f'M{cx},{cy-r} C{cx+k},{cy-k} {cx+k},{cy-k} {cx+r},{cy} C{cx+k},{cy+k} {cx+k},{cy+k} {cx},{cy+r} '
             f'C{cx-k},{cy+k} {cx-k},{cy+k} {cx-r},{cy} C{cx-k},{cy-k} {cx-k},{cy-k} {cx},{cy-r} Z', c)

ICONS = {}

# Image editing: a picture with mountains and sun.
ICONS['photocraft'] = ('NERDS Photo',
    s('M300,268 H724 Q792,268 792,336 V688 Q792,756 724,756 H300 Q232,756 232,688 V336 Q232,268 300,268 Z') +
    f('M262,726 L440,508 L548,640 L628,556 L762,726 Z') + f('M688,404 m-62,0 a62,62 0 1,0 124,0 a62,62 0 1,0 -124,0', L))

# Vector illustration: a pen nib on a curve with anchor points.
ICONS['vectorcraft'] = ('NERDS Vector',
    s('M200,820 C330,560 694,560 824,820', 40, L) +
    f('M432,196 H592 V276 H432 Z') +
    f('M408,308 H616 L716,520 L512,800 L308,520 Z') +
    s('M512,800 V600', 34, B, 'butt') + f('M512,560 m-44,0 a44,44 0 1,0 88,0 a44,44 0 1,0 -88,0', B) +
    f('M170,790 h60 v60 h-60 Z') + f('M794,790 h60 v60 h-60 Z'))

# Video editing: a clapperboard with a play button.
ICONS['filmcraft'] = ('NERDS Studio',
    f('M268,452 H756 Q796,452 796,492 V736 Q796,776 756,776 H268 Q228,776 228,736 V492 Q228,452 268,452 Z') +
    f('M454,532 L454,700 L600,616 Z', B) +
    f('M226,402 L758,262 Q794,253 803,289 L813,326 L247,476 L236,436 Q228,410 226,402 Z') +
    f('M330,375 L420,351 L364,450 L274,474 Z', B) + f('M500,330 L590,306 L534,405 L444,429 Z', B) +
    f('M670,285 L760,261 L704,360 L614,384 Z', B))

# Photo library and raw development: a camera aperture.
def aperture():
    cx, cy, R, r = 512, 512, 272, 108
    out = s(f'M{cx},{cy-R} a{R},{R} 0 1,0 0.1,0 Z', 56)
    for i in range(6):
        a = math.radians(60 * i - 90)
        x0, y0 = cx + r * math.cos(a), cy + r * math.sin(a)
        d = math.radians(60 * i - 90 + 90)            # tangent direction
        t = math.sqrt(R * R - r * r) - 20
        out += s(f'M{x0:.1f},{y0:.1f} L{x0 + t * math.cos(d):.1f},{y0 + t * math.sin(d):.1f}', 40)
    return out
ICONS['lightcraft'] = ('NERDS Light', aperture())

# PDFs: a page with a folded corner and a PDF band.
def page(x0, y0, x1, y1, fold=120, c=W):
    return f(f'M{x0+36},{y0} H{x1-fold} L{x1},{y0+fold} V{y1-36} Q{x1},{y1} {x1-36},{y1} H{x0+36} Q{x0},{y1} {x0},{y1-36} V{y0+36} Q{x0},{y0} {x0+36},{y0} Z', c)
def fold(x1, y0, fold=120):
    return f(f'M{x1-fold},{y0} V{y0+fold-24} Q{x1-fold},{y0+fold} {x1-fold+24},{y0+fold} H{x1} Z', L)
pdf_letters = (s('M352,700 V560 H408 Q456,560 456,606 Q456,652 408,652 H352', 34, W, 'butt', 'miter') +
               s('M494,700 V560 H536 Q604,560 604,630 Q604,700 536,700 H494 Z', 34, W, 'butt', 'miter') +
               s('M672,700 V560 H748 M672,628 H736', 34, W, 'butt', 'miter'))
ICONS['pdfcraft'] = ('NERDS PDF',
    page(268, 196, 756, 828) + fold(756, 196) + f('M228,520 H796 V740 H228 Z', B) + pdf_letters +
    s('M340,300 H560 M340,380 H660 M340,450 H660', 30, L))

# Motion graphics and visual effects: sparkles with a motion trail.
ICONS['effectcraft'] = ('NERDS Effects',
    star(470, 560, 290) + star(744, 296, 112, L) + star(752, 720, 64, L) + star(276, 256, 56, L))

# Page layout and publishing: an open two-page spread.
ICONS['designcraft'] = ('NERDS Layout',
    f('M212,276 H500 V772 H212 Q196,772 196,756 V292 Q196,276 212,276 Z') +
    f('M524,276 H812 Q828,276 828,292 V756 Q828,772 812,772 H524 Z') +
    f('M256,336 H440 V472 H256 Z', B) +
    s('M262,532 H434 M262,592 H434 M262,652 H400', 28, B, 'butt') +
    s('M584,342 H660 M584,402 H660 M584,462 H660 M584,522 H660 M584,582 H660 M584,642 H640', 28, B, 'butt') +
    s('M696,342 H768 M696,402 H768 M696,462 H768 M696,522 H768', 28, B, 'butt') + f('M696,560 H768 V700 H696 Z', L))

# Interface design and prototyping: an app window, a component and a cursor.
ICONS['figcraft'] = ('NERDS Proto',
    s('M262,232 H742 Q790,232 790,280 V650 Q790,698 742,698 H262 Q214,698 214,650 V280 Q214,232 262,232 Z', 48) +
    s('M214,330 H790', 40) + f('M268,268 m-0,0') +
    f('M286,392 H470 V500 H286 Z', L) + s('M286,566 H470', 36, L) + s('M532,410 H720 M532,480 H680', 36, W) +
    f('M560,520 L560,862 L640,782 L700,900 L762,870 L702,754 L812,746 Z', B) +
    f('M592,594 L592,790 L646,736 L696,836 L722,824 L672,724 L746,718 Z', W))

# Writing and document design: a page of text and a pencil.
ICONS['wordcraft'] = ('NERDS Type',
    page(232, 196, 700, 828) + fold(700, 196) +
    s('M300,330 H520 M300,420 H620 M300,510 H620 M300,600 H560 M300,690 H460', 36, B) +
    f('M820,420 L880,480 L600,760 L520,780 L540,700 Z', B) +
    f('M806,444 L856,494 L606,744 L556,756 L568,706 Z', W) + f('M556,756 L568,706 L606,744 Z', L))

# Recording, editing and mixing audio: a waveform.
def wave():
    hs = [120, 260, 400, 220, 520, 300, 420, 200, 120]
    out = ''
    for i, h in enumerate(hs):
        x = 512 + (i - 4) * 72
        out += s(f'M{x},{512 - h / 2} V{512 + h / 2}', 44, W if i % 2 == 0 else L)
    return out
ICONS['soundcraft'] = ('NERDS Sound', wave())

# Presentations: a screen on a stand with a chart.
ICONS['deckcraft'] = ('NERDS Deck',
    s('M188,236 H836', 52) +
    f('M224,268 H800 V632 Q800,664 768,664 H256 Q224,664 224,632 Z') +
    f('M300,560 H372 V470 H300 Z', B) + f('M410,560 H482 V400 H410 Z', B) + f('M520,560 H592 V330 H520 Z', B) +
    f('M630,560 H702 V430 H630 Z', L) +
    s('M512,664 V760 M512,700 L400,840 M512,700 L624,840', 48))

# Computer-aided design: a drafting compass drawing an arc.
ICONS['cadcraft'] = ('NERDS CAD',
    s('M236,736 Q512,880 788,736', 40, L) +
    s('M512,176 V250', 48) + f('M512,300 m-62,0 a62,62 0 1,0 124,0 a62,62 0 1,0 -124,0') +
    f('M512,300 m-24,0 a24,24 0 1,0 48,0 a24,24 0 1,0 -48,0', B) +
    s('M484,356 L322,782', 56, W, 'butt') + s('M540,356 L702,782', 56, W, 'butt') +
    s('M394,560 Q512,600 630,560', 34))

# Spreadsheets and data: a table with a header row.
ICONS['gridcraft'] = ('NERDS Grid',
    f('M276,228 H748 Q796,228 796,276 V748 Q796,796 748,796 H276 Q228,796 228,748 V276 Q228,228 276,228 Z') +
    f('M228,276 Q228,228 276,228 H748 Q796,228 796,276 V356 H228 Z', L) +
    s('M228,498 H796 M228,640 H796 M418,228 V796 M608,228 V796', 26, B, 'butt') +
    s('M228,356 H796', 26, B, 'butt'))

# The launcher: the white NERDS mark.
def launcher():
    # mark box 537..2142 x 97..2212 -> fit to 560 tall, centred.
    sc = 640 / 2115
    tx = 512 - (537 + 1605 / 2) * sc
    ty = 512 - (97 + 2115 / 2) * sc
    return f'<g transform="translate({tx:.2f},{ty:.2f}) scale({sc:.5f})" fill="{W}" fill-rule="evenodd">{MARK}</g>'
ICONS['nerds-launcher'] = ('NERDS Launcher', launcher())

out = Path(sys.argv[2])
for app, (name, body) in ICONS.items():
    (out / f'{app}.svg').write_text(tile(body, name, app))
print(len(ICONS), 'icons')
