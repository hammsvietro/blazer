defmodule Blazer.MixProject do
  use Mix.Project

  def project do
    [
      app: :blazer,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
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
      {:rustler, "~> 0.23.0"},
      {:jason, "~> 1.3"},
      {:credo, "~> 1.5", only: [:dev, :test], runtime: false}
    ]
  end
end
