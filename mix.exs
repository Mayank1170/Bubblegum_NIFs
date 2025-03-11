defmodule BubblegumElixir.MixProject do
  use Mix.Project

  def project do
    [
      app: :bubblegum_elixir,
      version: "0.1.0",
      elixir: "~> 1.14",
      # Removed :rustler compiler here
      compilers: Mix.compilers(),
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.29.1", runtime: false}

    ]
  end
end
