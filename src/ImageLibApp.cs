
using Avalonia.Media.Imaging;
using Avalonia;
using System.Collections.Generic;
using Avalonia.Controls;
using System;
using System.IO;

namespace img;
public class ImageApp
{
    Dictionary<string, Bitmap> _imgDict;
    List<string> _indexLst;
    int _index;
    string _saveDir;
    public ImageApp()
    {
        _indexLst = new List<string>();
        _imgDict = new Dictionary<string, Bitmap>();
        _index = 0;
        _saveDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.MyPictures), "ImgPreview");
        Directory.CreateDirectory(_saveDir);
    }

    // next img pnt
    public void next()
    {
        _index++;
        if (_index >= _indexLst.Count)
        {
            _index = 0;
        }
    }

    // back img pnt
    public void back()
    {
        _index--;
        if (_index < 0)
        {
            _index = _indexLst.Count - 1;
        }
    }

    // cleawr img
    public void clear()
    {
        _indexLst.Clear();
        _imgDict.Clear();
    }

    // my picture のImgPreviewにsave
    public void save(string dirPath)
    {
        var dir_name = Path.Combine(_saveDir, "dir_" + GetTimeStamp());
    }

    // my picture のImagePreviewからプロジェクト名でファイルリストをロード
    public void load(string dirPath)
    {
    }

    // get img
    public Bitmap getImg()

    {
        return _imgDict[_indexLst[_index]];
    }

    /// <summary>
    /// イメージを追加
    /// </summary>
    public void addImg(Bitmap bitmap)
    {
        // タイムスタンプを作成
        string timeStamp = GetTimeStamp();

        // ディクショナリに保管
        _imgDict.Add(timeStamp, bitmap);

        // indexリストの更新
        _indexLst.Clear();
        _indexLst.AddRange(_imgDict.Keys);

        // indexの更新
        _index = _indexLst.Count - 1;
    }

    // Utility -------------------------------------------------------------------------

    /// <summary>
    /// タイムスタンプの取得 
    /// </summary>
    public string GetTimeStamp()
    {
        _index = 0;
        string timestamp = DateTime.Now.ToString("yyyyMMdd_HHmmss") + "_" + _index;
        return timestamp;
    }

}
