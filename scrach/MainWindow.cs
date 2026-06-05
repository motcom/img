using System;
using Avalonia;
using Avalonia.Platform;
using Avalonia.Controls;
using Avalonia.Interactivity;

namespace scrach;

public class MainWindow : Window
{
    Button btnExe;
    TextBlock txtBlock;
    public MainWindow()
    {
        Width = 360;
        Height = 360;

        // Create
        btnExe = Ui.CreateButton("Exe");
        txtBlock = Ui.CreateTextBlock();

        // Expand
        btnExe.HorizontalAlignment = Avalonia.Layout.HorizontalAlignment.Stretch;



        // Layout
        Grid gridGetPath = new Grid
        {
            ColumnDefinitions = ColumnDefinitions.Parse("*"),
            RowDefinitions = RowDefinitions.Parse("Auto,*"),
        };

        Grid.SetColumn(btnExe, 0);
        Grid.SetRow(btnExe, 0);

        Grid.SetColumn(txtBlock, 0);
        Grid.SetRow(txtBlock, 1);

        gridGetPath.Children.Add(btnExe);
        gridGetPath.Children.Add(txtBlock);

        Content = gridGetPath;

        // Event
        btnExe.Click += OnBtnClicked;
    }

    /// <summary>
    /// Guiがある画像のピクセル数を返す
    /// </summary>
    /// <returns>PixelRect{width height}</returns>
    private PixelRect? WinScreenSize()
    {
        Screen? screen = this.Screens.ScreenFromWindow(this);
        if (screen is null) return null;
        PixelRect bounds = screen.Bounds;
        return bounds;
    }

    private void OnBtnClicked(object? sender, RoutedEventArgs e)
    {
    }
}
