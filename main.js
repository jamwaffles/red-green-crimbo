let counter = 0;

setInterval(function() {
  counter += 0.05;
  
  analogWrite(A5, Math.sin(counter) + 1, { freq : 1000 });  
  analogWrite(A6, Math.cos(counter) + 1, { freq : 1000 });
  
  analogWrite(A7, Math.sin(counter) + 1, { freq : 1000 });  
  analogWrite(B1, Math.cos(counter) + 1, { freq : 1000 });
  
  analogWrite(B10, Math.sin(counter) + 1, { freq : 1000 });  
  analogWrite(B13, Math.cos(counter) + 1, { freq : 1000 });
  
  analogWrite(B14, Math.sin(counter) + 1, { freq : 1000 });  
  analogWrite(B15, Math.cos(counter) + 1, { freq : 1000 });
}, 16);