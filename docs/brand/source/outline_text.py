"""Outline a line of text with fontTools: outline(font_path, axes, text, size, x, baseline, tracking_em)
returns (svg path d, bbox)."""
import io
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.boundsPen import BoundsPen

_cache = {}
def font(path, axes):
    key = (path, tuple(sorted(axes.items())))
    if key not in _cache:
        f = TTFont(path)
        f = instantiateVariableFont(f, axes, inplace=False)
        buf = io.BytesIO(); f.save(buf); buf.seek(0)
        _cache[key] = TTFont(buf)
    return _cache[key]

def outline(path, axes, text, size, x, baseline, tracking_em=0.0):
    f = font(path, axes)
    upm = f['head'].unitsPerEm
    cmap = f.getBestCmap(); gs = f.getGlyphSet(); hmtx = f['hmtx']
    s = size / upm
    spen = SVGPathPen(gs); bpen = BoundsPen(gs)
    cx = x
    for ch in text:
        g = cmap.get(ord(ch))
        if g is None: raise KeyError(ch)
        t = (s, 0, 0, -s, cx, baseline)
        gs[g].draw(TransformPen(spen, t)); gs[g].draw(TransformPen(bpen, t))
        cx += hmtx[g][0] * s + tracking_em * size
    return spen.getCommands(), bpen.bounds, cx
