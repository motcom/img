using Avalonia.Controls;
using Avalonia;
using Avalonia.Media;
namespace scrach;

public static class Ui
{
    public static ScrollViewer CreateScrollViewer()
    {
        return new ScrollViewer
        {
            HorizontalScrollBarVisibility = Avalonia.Controls.Primitives.ScrollBarVisibility.Auto,
            VerticalScrollBarVisibility = Avalonia.Controls.Primitives.ScrollBarVisibility.Auto,
        };

    }
    public static TextBlock CreateTextBlock()
    {
        return new TextBlock
        {
            FontSize = 10,
            Margin = new Thickness(2),
            Padding = new Thickness(3),
        };

    }

    public static TextBox CreateTextBox()
    {
        return new TextBox
        {
            FontSize = 10,
            Height = 20,
            MinHeight = 20,
            Margin = new Thickness(2),
            Padding = new Thickness(3),
        };
    }

    public static TextBox CreateIntTextBox(int defalutVal)
    {
        return new TextBox
        {
            FontSize = 10,
            Height = 20,
            MinHeight = 20,
            Margin = new Thickness(2),
            Padding = new Thickness(3),
            Text = defalutVal.ToString(),
        };
    }

    public static Label CreateLabel(string labelName)
    {
        return new Label
        {
            FontSize = 10,
            Margin = new Thickness(2),
            Padding = new Thickness(3),
            Content = labelName,
            Height = 20,
            MinHeight = 20,
        };
    }

    public static Button CreateButton(string btnName)
    {
        return new Button
        {
            FontSize = 10,
            Content = btnName,
            Height = 20,
            MinHeight = 20,
            Margin = new Thickness(2),
            Padding = new Thickness(3),
            Background = Brushes.AliceBlue,
            VerticalContentAlignment = Avalonia.Layout.VerticalAlignment.Center,
            HorizontalContentAlignment = Avalonia.Layout.HorizontalAlignment.Center,
        };
    }

    public static Border CreateBorder(double borderWidth)
    {
        return new Border
        {
            Height = 10,
            Child = new Border
            {
                Height = borderWidth,
                Background = Brushes.Gray,
                BorderThickness = new Thickness(borderWidth),
                VerticalAlignment = Avalonia.Layout.VerticalAlignment.Center,
            },
        };
    }
}
