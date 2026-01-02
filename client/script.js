function formatAMPM(date) {
  var hours = date.getHours();
  var minutes = date.getMinutes();
  var ampm = hours >= 12 ? 'PM' : 'AM';
  hours = hours % 12;
  hours = hours ? hours : 12; // the hour '0' should be '12'
  minutes = minutes < 10 ? '0' + minutes : minutes;
  var strTime = hours + ':' + minutes + ' ' + ampm;
  return strTime;
}

function resetChat() {
  $("ul").empty();
}

function insertChat(who, text, time) {
  if (time === undefined) {
    time = 0;
  }
  var control = "";
  var date = formatAMPM(new Date());

  if (who == "me") {
    control = '<li style="width:100%">' +
      '<div class="msj macro">' +
      //'<div class="avatar"><img class="img-circle" style="width:100%;" src="' + me.avatar + '" /></div>' +
      '<div class="text text-l">' +
      '<p>' + text + '</p>' +
      '<p><small>' + date + '</small></p>' +
      '</div>' +
      '</div>' +
      '</li>';
  } else {
    control = '<li style="width:100%;">' +
      '<div class="msj-rta macro">' +
      '<div class="text text-r">' +
      '<p>' + text + '</p>' +
      '<p><small>' + date + '</small></p>' +
      '</div>' +
      //'<div class="avatar" style="padding:0px 0px 0px 10px !important"><img class="img-circle" style="width:100%;" src="' + you.avatar + '" /></div>' +
      '</li>';
  }
  setTimeout(
    function () {
      $("ul").append(control).scrollTop($("ul").prop('scrollHeight'));
    }, time);

}

// TODO: change to "connect"
const socket = new WebSocket("ws://localhost:7000/v1/ws");

function sendMessage() {
  const text = $("#message-input").val();
  socket.send(text);
  insertChat("me", text);
}

// Connection opened
socket.addEventListener("open", (event) => {
  socket.send("Hello Server!");
});

// Listen for messages
socket.addEventListener("message", (event) => {
  insertChat("server", event.data, new Date());
});
