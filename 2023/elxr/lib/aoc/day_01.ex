defmodule AoC.Day01 do
  def part1(input) do
    input
    |> parse_numbers()
    |> Enum.sum()
  end

  defp get_numbers(line) do
    numbers = Enum.filter(line, fn char -> char in ?0..?9 end)
    first = hd(numbers)
    last = hd(Enum.reverse(numbers))
    [first, last] |> List.to_integer()
  end

  defp parse_numbers(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_charlist/1)
    |> Enum.map(&get_numbers/1)
  end

  def part2(input) do
    input
    |> translate_text_to_numbers()
    |> parse_numbers()
    |> Enum.sum()
  end

  defp translate_text_to_numbers("one" <> rest),
    do: "1" <> translate_text_to_numbers("e" <> rest)

  defp translate_text_to_numbers("two" <> rest),
    do: "2" <> translate_text_to_numbers("o" <> rest)

  defp translate_text_to_numbers("three" <> rest),
    do: "3" <> translate_text_to_numbers("e" <> rest)

  defp translate_text_to_numbers("four" <> rest),
    do: "4" <> translate_text_to_numbers("r" <> rest)

  defp translate_text_to_numbers("five" <> rest),
    do: "5" <> translate_text_to_numbers("e" <> rest)

  defp translate_text_to_numbers("six" <> rest),
    do: "6" <> translate_text_to_numbers("x" <> rest)

  defp translate_text_to_numbers("seven" <> rest),
    do: "7" <> translate_text_to_numbers("n" <> rest)

  defp translate_text_to_numbers("eight" <> rest),
    do: "8" <> translate_text_to_numbers("t" <> rest)

  defp translate_text_to_numbers("nine" <> rest),
    do: "9" <> translate_text_to_numbers("e" <> rest)

  defp translate_text_to_numbers(<<char, rest::binary>>),
    do: <<char>> <> translate_text_to_numbers(rest)

  defp translate_text_to_numbers(""), do: ""
end
