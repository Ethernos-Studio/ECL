// ECL标准库 - 字符串函数
func strlen(s) {
    var len = 0;
    var i = 0;
    while i < len(s) {
        len = len + 1;
        i = i + 1;
    }
    return len;
}

func substring(s, start, end) {
    var result = "";
    var i = start;
    while i < end && i < len(s) {
        result = result + s[i];
        i = i + 1;
    }
    return result;
}

func contains(s, substr) {
    var i = 0;
    while i <= len(s) - len(substr) {
        var j = 0;
        var found = true;
        while j < len(substr) {
            if s[i + j] != substr[j] {
                found = false;
                break;
            }
            j = j + 1;
        }
        if found {
            return true;
        }
        i = i + 1;
    }
    return false;
}

func starts_with(s, prefix) {
    if len(prefix) > len(s) {
        return false;
    }
    var i = 0;
    while i < len(prefix) {
        if s[i] != prefix[i] {
            return false;
        }
        i = i + 1;
    }
    return true;
}

func ends_with(s, suffix) {
    if len(suffix) > len(s) {
        return false;
    }
    var i = len(s) - len(suffix);
    var j = 0;
    while j < len(suffix) {
        if s[i + j] != suffix[j] {
            return false;
        }
        j = j + 1;
    }
    return true;
}