function fromJs(listInput) {
  var listInput = arrayfromargs(arguments);
  var p = this.patcher;
  var myObject = p.getnamed("___rytm-help-more");
  if (myObject) {
    myObject.message(listInput);
  } else {
    post("Error: Object not found.\n");
  }
}
