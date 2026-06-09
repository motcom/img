using Avalonia.Controls;
using Avalonia.Input;
namespace scrach;

public class SaveDirNameWindow : Window
{
    Label label;
    TextBox txtBox;
    public SaveDirNameWindow()
    {

        // create
        label = Ui.CreateLabel("FileName:");
        txtBox = Ui.CreateTextBox();


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


        grid.Children.Add(label);
        grid.Children.Add(txtBox);

        Content = grid;


        // イベント
        // フォーカスをテキストボックスに移す
        Opened += (_, _) => txtBox.Focus();

        // エンターを押して終了
        txtBox.KeyDown += (o, e) =>
        {
            if (e.Key == Key.Enter)
                this.Close(txtBox.Text);
        };


    }

}
