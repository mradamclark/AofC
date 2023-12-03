defmodule Mix.Tasks.Day do
  use Mix.Task

  @shortdoc "Run Day"
  def run(args) do
    day = Enum.at(args, 0, 1)
    part = Enum.at(args, 1, 1)

    input = AoC.Loader.load(day)

    module_name = String.to_atom("Elixir.AoC.Day#{String.pad_leading(day, 2, "0")}")
    fun_name = String.to_atom("part#{part}")

    result = apply(module_name, fun_name, [input])
    IO.inspect(result, label: "Part #{part} results:")
  end
end
