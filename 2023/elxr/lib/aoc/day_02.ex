defmodule AoC.Day02 do
  @moduledoc false
  alias AoC.Day02.GameParser

  import NimbleParsec

  def part1(input) do
    bag = %{"red" => 12, "green" => 13, "blue" => 14}

    input
    |> parse_input()
    |> Enum.with_index(1)
    |> Enum.filter(fn {line, _} -> possible_cubes(line, bag) end)
    |> Enum.map(fn {_, i} -> i end)
    |> Enum.sum()
  end

  def part2(input) do
    input
    |> parse_input()
    |> Enum.map(fn game -> power_of_smallest(game) end)
    |> Enum.sum()
  end

  defp power_of_smallest(game) do
    handfuls = GameParser.parse(game)

    cubes =
      elem(handfuls, 1)
      |> Enum.map(fn {_, cubes} ->
        cubes = Enum.chunk_every(cubes, 2)
        Enum.into(cubes, [], fn [count, colour] -> {colour, count} end)
      end)
      |> Enum.flat_map(fn x -> x end)
      |> Enum.group_by(&elem(&1, 0), &elem(&1, 1))

    Enum.max(cubes["red"]) * Enum.max(cubes["green"]) * Enum.max(cubes["blue"])
  end

  defp possible_cubes(line, bag) do
    handfuls = GameParser.parse(line)

    elem(handfuls, 1)
    |> Enum.map(fn {_, cubes} ->
      cubes = Enum.chunk_every(cubes, 2)
      Enum.into(cubes, %{}, fn [count, colour] -> {colour, count} end)
    end)
    |> Enum.all?(fn handful ->
      Map.get(handful, "red", 0) <= bag["red"] &&
        Map.get(handful, "green", 0) <= bag["green"] &&
        Map.get(handful, "blue", 0) <= bag["blue"]
    end)
  end

  defp parse_input(input) do
    String.split(input, "\n", trim: true)
  end

  defmodule GameParser do
    @moduledoc false
    import NimbleParsec
    # Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red

    cube =
      integer(min: 1)
      |> ignore(string(" "))
      |> choice([string("red"), string("green"), string("blue")])

    cubes =
      times(cube |> ignore(choice([optional(string(", ")), eos()])), min: 1, max: 3)
      |> tag("cubes")

    defparsec(
      :parse,
      ignore(
        string("Game ")
        |> integer(min: 1, max: 10)
        |> string(": ")
      )
      |> times(cubes |> ignore(optional(choice([string("; "), eos()]))), min: 1)
    )
  end
end
