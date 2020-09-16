<?php
  $value = 0;

  if($value%2){
    echo "<h1>Odd</h1>";
  }else{
    echo "<h1>Even</h1>";
  }
  $x = 3243;
  $y = -345823;
  $sub = abs($y-$x);
  echo "<h1>".$sub."</h1>";

  $score = 30;
  $greaterThen80 = $score >= 80;
  $greaterThen60 = $score >= 60;
  $greaterThen50 = $score >= 50;
  $greaterThen40 = $score >= 40;
  switch ($score) {
    case $greaterThen80 :
      echo "<h1>Very Good</h1>";
      break;
    case $greaterThen60:
      echo "<h1>Good</h1>";
      break;
    case $greaterThen50:
      echo "<h1>Average</h1>";
      break;
    case $greaterThen40:
      echo "<h1>Just Passed</h1>";
      break;
    default:
      echo "<h1>Go Study </h1>";
      break;
  }
  function final_salary($salary){
    if($salary <= 2000){
      $salary += $salary*0.1+$salary*0.2;
    }
    elseif($salary <= 5000){
      $salary += $salary + $salary*0.2+$salary*0.2;
    }
    elseif($salary <= 10000){
      $salary += $salary*0.33+$salary*0.4;
    }
    elseif($salary > 10000){
      $salary += $salary*0.5+$salary*0.5;
    }
    return $salary;
  }
  echo "<h1>".final_salary(1000)."</h1>";

  function paid_amount($pa){
    if($pa >= 5000){
      $pa -= 0.05*$pa;
    }
    elseif($pa >= 4000){
      $pa -= 0.04*$pa;
    }
    elseif ($pa >= 3000) {
      $pa -= 0.03*$pa;
    }
    elseif ($pa >= 2000) {
      $pa -= 0.02*$pa;
    }
    elseif($pa >= 1000){
      $pa -= 0.01*$pa;
    }else{
      return $pa;
    }
    return $pa;
  }

  echo "<h1>".paid_amount(999)."</h1>";

 ?>
