using System;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media.Imaging;
using Avalonia.Input.Platform;
using MsBox.Avalonia;
using MsBox.Avalonia.Enums;
using Avalonia.Media;

namespace img;

public class MainWindow : Window
{
    TopLevel? _topLevel;
    Image _imageCtrl;
    Border _resizeGrip;
    ImageApp _imgApp;

    // Resize用
    Point? _resizeStart;
    Size _startSize;

    // コンストラクタ
    public MainWindow()
    {
        // このウィンドウのプロパティ
        Width = 360;
        Height = 360;
        MinWidth = 100;
        MinHeight = 100;
        CanResize = true;

        this.Topmost = true;
        this.ExtendClientAreaToDecorationsHint = true;
        this.WindowDecorations = WindowDecorations.None;

        // 生成 -----------------------------------------------------------
        _topLevel = TopLevel.GetTopLevel(this);
        _imgApp = new ImageApp();
        _imageCtrl = new Image
        {
            Stretch = Stretch.Uniform
        };

        _resizeGrip = new Border
        {
            Width = 18,
            Height = 18,
            Background = Brushes.Transparent,
            Cursor = new Cursor(StandardCursorType.SizeNorthSouth),
            HorizontalAlignment = Avalonia.Layout.HorizontalAlignment.Right,
            VerticalAlignment = Avalonia.Layout.VerticalAlignment.Bottom
        };

        // Layout ----------------------------------------------------
        Grid root = new Grid();
        root.Children.Add(_imageCtrl);
        root.Children.Add(_resizeGrip);
        this.Content = root; // 親にコントロールを渡す

        // イベントハンドラ ----------------------------------------------------------------------

        // resize Event -----------------------------------------------

        // マウスボタンプレス リサイズスタート
        _resizeGrip.PointerPressed += (_, e) =>
        {
            e.Handled = true; // このイベントが呼ばれたら個々で切り上げる
            _resizeStart = e.GetPosition(this);
            _startSize = new Size(Width, Height);
            e.Pointer.Capture(_resizeGrip);
        };

        // マウスムーブ
        _resizeGrip.PointerMoved += (_, e) =>
        {
            if (_resizeStart == null) return;
            Point pos = e.GetPosition(this);
            double dx = pos.X - _resizeStart.Value.X;
            double dy = pos.Y - _resizeStart.Value.Y;

            Width = Math.Max(MinWidth, _startSize.Width + dx);
            Height = Math.Max(MinHeight, _startSize.Height + dy);
        };

        // マウスリリース
        _resizeGrip.PointerReleased += (_, e) =>
        {
            _resizeStart = null;
            e.Pointer.Capture(null);
        };

        // キーイベントハンドラ
        this.KeyDown += OnKeyDown;

        // マウスのドラッグを追加(マウスボタンプレスの e.Handledがtrue の場合スキップ)
        this.PointerPressed += (_, e) =>
        {
            BeginMoveDrag(e);
        };

        // ----------------------------------------------------------------------------------------
    }

    // EventHandler ---------------------------------------------------------------------------------------

    // キーイベントハンドラ
    private void OnKeyDown(object? sender, KeyEventArgs e)
    {
        Console.WriteLine($"key={e.Key}");
        switch (e.Key)
        {

            // ペースト
            case Key.V when e.KeyModifiers.HasFlag(KeyModifiers.Control):
            case Key.P:
                OnPaste();
                break;

            // トップトグル
            case Key.Tab:
            case Key.T:
                OnTopToggle();
                break;

            // 閉じる
            case Key.Escape:
            case Key.Q:
                OnCloseWindow();
                break;

            // Next
            case Key.Right:
            case Key.L:
                OnNextImage();
                break;

            // Back
            case Key.H:
            case Key.Left:
                OnBackImage();
                break;

            // Save
            case Key.S when e.KeyModifiers.HasFlag(KeyModifiers.Control):
            case Key.S:
                OnSave();
                break;

            case Key.O when e.KeyModifiers.HasFlag(KeyModifiers.Control):
            case Key.O:
                OnLoad();
                break;

        }
    }


    private async void OnSave()
    {
        SaveDirNameWindow wnd = new();
        string? fileName = await wnd.ShowDialog<string?>(this);
        Console.WriteLine($"{fileName}");

        _imgApp.save(fileName);

    }

    private void OnLoad()
    {
        Console.WriteLine("Load");
    }

    private void OnBackImage()
    {
        _imgApp.next();
        windowUpdate();
    }

    private void OnNextImage()
    {
        _imgApp.back();
        windowUpdate();
    }

    // ウィンドウを閉じる
    private async void OnCloseWindow()
    {
        Console.WriteLine("on close windows");
        var box =
            MessageBoxManager.GetMessageBoxStandard
                ("", "本当に閉じますか？", ButtonEnum.YesNo);
        var result = await box.ShowWindowDialogAsync(this);
        if (result == ButtonResult.Yes) this.Close();
    }

    // トップトグル
    private void OnTopToggle()
    {
        Console.WriteLine("on top toggle");
        if (this.Topmost)
        {
            this.Topmost = false;
        }
        else
        {
            this.Topmost = true;
        }
    }

    // ペースト
    async private void OnPaste()
    {
        Console.Out.WriteLine("on paste");

        if (_topLevel is not null)
        {
            // クリップボードから画像を取得
            var clipboard = _topLevel.Clipboard;
            if (clipboard is null) return;
            Bitmap? bitmap = await clipboard.TryGetBitmapAsync();

            // ImgeAppに画像を渡す
            if (bitmap is null) return;
            _imgApp.addImg(bitmap);
            windowUpdate();

        }

    }

    // Utility ------------------------------------------------------------------
    // ImgAppでの現在の画像をウィンドウに出力
    private void windowUpdate()
    {
        // _imgAppより現在のイメージの取得
        Bitmap bitmap = _imgApp.getImg();

        // 画像の更新
        this._imageCtrl.Source = bitmap;
    }


}
