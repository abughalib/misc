</<?php
echo "<h1></h1>";
$num = 123;
$sumOfDigits = 0;
while($num != 0){
  $sumOfDigits+= $num%10;
  $num /= 10;
}
echo "<h1>Sum of Digits: </h1><h2>".$sumOfDigits."</h2><hr>";
//echo "<h1>Prime Number Test</h1>";
$number = 73;
$isPrime = true;
for($i = 2; $i <=$number/2; $i++){
  if($number%$i == 0) $isPrime = false;
}
if($isPrime){
  echo "<h2>".$number."</h2><h1> is Prime Number</h1>";
}else{
  echo "<h1>Not A Prime Number</h1>";
}
echo "<hr>";
function factorial($num){
  $factValue = 1;
  for($i = 1; $i <= $num; $i++){
    $factValue*=$i;
  }
  return $factValue;
}
echo "<h1>Factorial </h1><h2>".factorial(5)."</h2><hr>";

function sumOfCubeOfDigit($number){
  $sumOfDigitsCube=0;
  while($number != 0){
    $sumOfDigitsCube+= pow($number%10, 3);
    $number /= 10;
  }
  return $sumOfDigitsCube;
}
echo "<h1>Arm Strong Number</h1>";
$armNums = [];
for($i = 100; $i <= 999; $i++){
  if($i == sumOfCubeOfDigit($i)){
    array_push($armNums, $i);
  }
}
foreach ($armNums as $key => $value) {
  echo "<h2>".$value."</h2>";
}
echo "<hr>";

$users = ['Abugh'];

for($i = 'A'; $i < 'Z'; $i++){
  array_push($users, $i);
}
echo "<h1>Printing Users in Array</h1>";
echo "<h2>";
foreach ($users as $key => $value) {
  echo $value.", ";
}
echo "</h2>";
echo "<hr>";
echo "<h1>Number -> Square and Cube List</h1>";
echo "<table border='2'>";
echo "<td>Number</td><td>Square</td><td>Cube</td>";
for($i=0; $i<=10; $i++){
  echo "<tr><td>".$i."</td><td>".pow($i, 2)."</td><td>".pow($i, 3)."</td></tr>";
}
echo "</table><br><br>";
echo "<hr>";
$arrayOfStudent = array(
  array('Abugh', 'B.tech', 'BTECH01', '+9179......50'),
  array('Ajay', 'BCA', 'BCA01', '+9179......50'),
  array('Vijay','MTECH','BTECH01','+9179......50'),
  array('Ranvijay','MTECH','MTECH02','+9179......50'),
  array('Digvijay','MTECH', 'MTECH01','+9179......50')
);
echo "<h1>Student Table</h1>";
echo "<table border='2'>";
echo "<tr><td>S.No</td><td>Name</td><td>Course</td><td>Batch</td><td>Contact Number</td></tr>";
for($i= 0; $i < 5; $i++){
  echo "<tr><td>".($i+1)."</td><td>".$arrayOfStudent[$i][0].
  "</td><td>".$arrayOfStudent[$i][1]."</td><td>".
  $arrayOfStudent[$i][2]."</td><td>".$arrayOfStudent[$i][3]."</td></tr>";
}
echo "</table>";
// echo "<h1>Student Table</h1>";
// echo "<table border='1'>";
// echo "<th colspan='0'>Name<td>Age</td></th>";
// for ($i=1; $i <=5; $i++) {
//   echo "<tr><td>".$i."</td><td>".$arrayOfStudent[$i][0]."</td></tr>";
// }
echo "</table>";
echo "<hr><hr><hr><h1 style='color:red;'>EOF</h1>";
 ?>
