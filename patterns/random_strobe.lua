require "patterns/util"

options.color = {
  default = 0xffffff,
  desc = "Color"
}

function setup()
end

function update(dt)
  values = {}

  for i = 1, element_count do
    local x = math.random()

    if x > 0.90 then
      values[i] = options.color.value
    else
      values[i] = 0
    end
  end

  return values
end
