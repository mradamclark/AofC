defmodule AoC.MixProject do
  use Mix.Project

  def project do
    [
      app: :aoc,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      env: [
        cookie: ""
      ]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:benchee, "~> 1.2"},
      {:httpoison, "~> 2.2"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:nimble_parsec, "~> 1.2"}
    ]
  end
end
