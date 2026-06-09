
using Avalonia.Media.Imaging;
using System.Collections.Generic;
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
        _saveDir = Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.MyPictures),
                "ImgPreview");
        Directory.CreateDirectory(_saveDir);
    }

    /// <summary>
    /// イメージを追加
    /// </summary>
    public void AddImg(Bitmap bitmap)
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

    // get img
    public Bitmap GetImg()
    {
        return _imgDict[_indexLst[_index]];
    }


    // Utility -------------------------------------------------------------------------
    /// <summary>
    /// タイムスタンプの取得 
    /// </summary>
    public string GetTimeStamp()
    {
        string timestamp = DateTime.Now.ToString("yyyyMMdd_HHmmss_fff");
        return timestamp;
    }

    // next img pnt
    public void Next()
    {
        _index++;
        if (_index >= _indexLst.Count)
        {
            _index = 0;
        }
    }

    // back img pnt
    public void Back()
    {
        _index--;
        if (_index < 0)
        {
            _index = _indexLst.Count - 1;
        }
    }

    // cleawr img
    public void Clear()
    {
        _indexLst.Clear();
        _imgDict.Clear();
    }

    // my picture のImgPreviewにsave
    public void Save(string? dirNamePrefix)
    {

        string prefix;
        if (dirNamePrefix is null or "")
        {

            prefix = "dir_" + GetTimeStamp();
        }
        else
        {
            prefix = dirNamePrefix;
        }

        var dirName = Path.Combine(_saveDir, prefix);
        Directory.CreateDirectory(dirName);

        foreach (var key in _imgDict.Keys)
        {
            var fileName = Path.Combine(dirName, key + ".png");
            Console.WriteLine($"FileName={fileName}");
            _imgDict[key].Save(fileName);
        }


    }

    // my picture のImagePreviewからプロジェクト名でファイルリストをロード
    public void Load()
    {
        string[] dirs = Directory.GetDirectories(_saveDir);


    }


}
