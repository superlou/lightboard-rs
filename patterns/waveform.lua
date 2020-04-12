require "patterns/util"

options.color = {
  default = 0xffffff,
  desc = "Color"
}

options.frequency = {
  default = 1.0,
  desc = "Waveform frequency"
}

options.reverse = {
  default = false,
  desc = "Direction of chase"
}

options.reflect = {
  default = 0,
  desc = "Create a symmetric pattern (todo arbitrary number of reflections)"
}

local pi2 = 2 * math.pi
local phase_map = {}

function setup()
  t = 0

  for i = 1, element_count do
    phase_map[i] = i - 1

    if options.reverse.value then
      phase_map[i] = -1 * phase_map[i]
    end
  end

  -- todo General case that doesn't require even number of elements
  if options.reflect.value == 1 then
    for i = 1, #phase_map / 2 do
      phase_map[#phase_map - i + 1] = phase_map[i]
    end
  end
end

function update(dt)
  t = t + dt
  local values = {}

  for i = 1, element_count do
    local alpha = 0.5 * math.sin(pi2 * options.frequency.value * t - phase_map[i]) + 0.5
    local r, g, b = int_as_rgb(options.color.value)
    values[i] = rgb_as_int(r * alpha, g * alpha, b * alpha)
  end

  return values
end
