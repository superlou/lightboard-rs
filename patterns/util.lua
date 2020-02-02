function round(x)
  return math.floor(x + 0.5)
end

function rgb_as_int(r, g, b)
  return (round(r) << 16) + (round(g) << 8) + round(b)
end

function int_as_rgb(val)
  return (val >> 8) & 0xff, (val >> 8) & 0xff, val & 0xff
end
