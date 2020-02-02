options.color = {
  default = 0xffffff,
  desc = "Color"
}

function setup()
end

function update(dt)
  values = {}

  for i = 1, element_count do
    values[i] = options.color.value
  end

  return values
end
