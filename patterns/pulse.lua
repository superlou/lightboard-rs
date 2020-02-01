color = 0x123456
t = 0

actual_color = {}
actual_color.r = 0xfe
actual_color.g = 0x87
actual_color.b = 0x32

function round(x)
  return math.floor(x + 0.5)
end

function rgb_as_int(r, g, b)
  return (round(r) << 16) + (round(g) << 8) + round(b)
end

function setup()
  options = {}
  options.color = {
    default = color,
    desc = "Color"
  }
  return options
end

function update(dt)
  values = {}
  t = t + dt

  local pi2 = 2 * math.pi

  for i = 1, element_count do
    local phase = (i - 1) / element_count
    alpha = 0.5 * math.sin(pi2 * 1.0 * t + phase) + 0.5
    values[i] = rgb_as_int(actual_color.r * alpha,
                           actual_color.g * alpha,
                           actual_color.b * alpha)
  end

  return values
end
