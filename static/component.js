String.prototype.setDefault = function (defaultValue) {
    switch (this) {
        case "":
        case "null":
        case null:
        case undefined:
        case "NULL":
        case "Null":
        case "undefined":
            return defaultValue;
        default:
            return str;
    }
}