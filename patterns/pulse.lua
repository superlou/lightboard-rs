require "patterns/util"

options.color = {
  default = 0xffffff,
  desc = "Color"
}

function setup()
  t = 0
end

function update(dt)
  values = {}
  t = t + dt

  local pi2 = 2 * math.pi

  for i = 1, element_count do
    local phase = i - 1
    local alpha = 0.5 * math.sin(pi2 * 1.0 * t - phase) + 0.5
    local r, g, b = int_as_rgb(options.color.value)
    values[i] = rgb_as_int(r * alpha, g * alpha, b * alpha)
  end

  return values
end
