
using System;
using Avalonia.Controls;
using Avalonia.Input;

namespace scrach;

class LoadDirNameWithSelector : Window
{
    ListBox lstBox = new();

    public LoadDirNameWithSelector(string[] dirNames)
    {
        // プロパティ
        this.SizeToContent = SizeToContent.WidthAndHeight;
        this.MinWidth = 300;
        this.MinHeight = 300;
        this.WindowStartupLocation = WindowStartupLocation.CenterOwner;

        foreach (var dir in dirNames)
        {
            lstBox.Items.Add(dir);
        }


        Content = lstBox;

        AddHandler(KeyDownEvent, OnKeyDown, Avalonia.Interactivity.RoutingStrategies.Tunnel);
    }

    private void OnKeyDown(object? sender, KeyEventArgs e)
    {
        switch (e.Key)
        {
            case Key.Down:
            case Key.J:
                lstBox.SelectedIndex++;
                break;
            case Key.Up:
            case Key.K:
                if (lstBox.SelectedIndex < 0)
                {
                    lstBox.SelectedIndex = lstBox.Items.Count - 1;
                }
                else
                {
                    lstBox.SelectedIndex--;
                }
                break;
            case Key.Enter:
                Close(lstBox.SelectedItem as string);
                break;

        }
    }
}
