defmodule AoC.Day03 do
  def part1(input) do
    nums = get_nums(input)
    symbols = get_symbols(input)

    nums
    |> Stream.filter(fn {row_span, col_span, _n} ->
      is_touching_symbol?(row_span, col_span, symbols)
    end)
    |> Stream.map(&elem(&1, 2))
    |> Enum.sum()
  end

  def part2(input) do
    nums = get_nums(input)
    gears = get_gears(input)

    gears
    |> Stream.map(fn {i, j} ->
      case Enum.filter(nums, fn {row_span, col_span, _n} -> i in row_span and j in col_span end) do
        [a, b] -> elem(a, 2) * elem(b, 2)
        _ -> 0
      end
    end)
    |> Enum.sum()
  end

  defp into_lines(input), do: String.split(input, "\n", trim: true) |> Enum.with_index()

  defp is_touching_symbol?(row_span, col_span, symbols) do
    for i <- row_span,
        j <- col_span,
        reduce: false,
        do: (acc -> acc || {i, j} in symbols)
  end

  defp get_gears(input) do
    input
    |> into_lines()
    |> Stream.flat_map(fn {line, i} ->
      Regex.scan(~r/\*/, line, return: :index)
      |> List.flatten()
      |> Enum.map(fn {j, _} -> {i, j} end)
    end)
    |> MapSet.new()
    |> IO.inspect(label: "gears")
  end

  defp get_symbols(input) do
    input
    |> into_lines()
    |> Stream.flat_map(fn {line, i} ->
      Regex.scan(~r/[^a-zA-Z0-9\.]/, line, return: :index)
      |> List.flatten()
      |> Enum.map(fn {j, _} -> {i, j} end)
    end)
    |> MapSet.new()
    |> IO.inspect(label: "symbols")
  end

  defp get_nums(input) do
    input
    |> into_lines()
    |> Enum.flat_map(fn {line, i} ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.map(fn {j, len} ->
        {(i - 1)..(i + 1)//1, (j - 1)..(j + len)//1,
         String.to_integer(String.slice(line, j, len))}
      end)
    end)
    |> IO.inspect(label: "numbers")
  end
end
