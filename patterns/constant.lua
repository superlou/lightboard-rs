color = 0x123456

function setup()
  options = {}
  options.color = {
    default = color,
    desc = "Color"
  }
  return options
end

function update()
  values = {}

  for i = 1, element_count do
    values[i] = color
  end

  return values
end
