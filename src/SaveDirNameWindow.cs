using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Input;

namespace img;

public class SaveDirNameWindow : Window
{
    Label label;
    TextBox txtBox;
    Button exeBtn;
    public SaveDirNameWindow()
    {

        // create
        label = Ui.CreateLabel("FileName:");
        txtBox = Ui.CreateTextBox();
        exeBtn = Ui.CreateButton("Save");


        // propety 
        this.Title = "SetDirectoryName";
        this.SizeToContent = SizeToContent.WidthAndHeight;
        this.WindowStartupLocation = WindowStartupLocation.CenterOwner;

        this.Topmost = true;
        this.txtBox.MinWidth = 200;

        //layout
        Grid grid = new Grid
        {
            ColumnDefinitions = ColumnDefinitions.Parse("auto,*,auto"),
            RowDefinitions = RowDefinitions.Parse("auto")
        };

        Grid.SetColumn(label, 0);
        Grid.SetRow(label, 0);

        Grid.SetColumn(txtBox, 1);
        Grid.SetRow(txtBox, 0);

        Grid.SetColumn(exeBtn, 2);
        Grid.SetRow(exeBtn, 0);

        grid.Children.Add(label);
        grid.Children.Add(txtBox);
        grid.Children.Add(exeBtn);

        Content = grid;

        txtBox.KeyDown += (o, e) =>
        {
            if (e.Key == Key.Enter)
                this.Close(txtBox.Text);
        };

        exeBtn.Click += OnExeBtn;

    }

    private void OnExeBtn(object? sender, RoutedEventArgs e)
    {
        this.Close(txtBox.Text);
    }
}
