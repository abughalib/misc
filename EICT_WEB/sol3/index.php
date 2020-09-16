<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sign Up Form</title>
    <link rel="stylesheet" href="main.css">
</head>
<body>
    <div class="form">
        <h3>Sign Up</h3>
        <div class="element">
            <label>First Name</label>
            <input type="text" class="Tinput" placeholder="Enter First Name...">
        </div>
        <div class="element">
            <label>Last Name</label>
            <input type="text" class="Tinput" placeholder="Enter Last Name...">
        </div>
        <div class="element">
            <label>Screen Name</label>
            <input type="text" class="Tinput" placeholder="Enter Screen Name...">
        </div>
        <div class="element">
            <label>Date of Birth</label>
            <select class="select date-month">
                <?php
                    $months = ['January', 'February', 'March', 'April', 'May', 
                    'June', 'July', 'August', 'September', 'October', 'November', 'December'];
                    foreach($months as $key => $value) {
                        echo "<option>".$value."</option>";
                    }
                ?>
            </select>
            <select class="select date-day">
                <?php
                    for($i = 1; $i < 31; $i++){
                        echo "<option>".$i."</option>";
                    }
                ?>
            </select>
            <select class="select date-year">
                <?php
                    for($i = 1900; $i < 2100; $i++){
                        echo "<option>".$i."</option>";
                    }
                ?>
            </select>
        </div>
        <div class="element radio">
            <label>Gender</label>
            <input type="radio" name="gender" style="margin-right: 20px;">
            <label>Male</label>
            <input type="radio" name="gender" style="margin-right: 20px;">
            <label>Female</label>
        </div>
        <div class="element">
            <label>Country</label>
            <select class="Tinput select">
                <option>Afganistan</option>
                <option>Austria</option>
                <option>Bangladesh</option>
                <option>Bhutan</option>
                <option>Brazil</option>
                <option>China</option>
                <option>France</option>
                <option>India</option>
                <option>Manmar</option>
                <option>Maldives</option>
                <option>Nepal</option>
                <option>Pakistan</option>
                <option>Russia</option>
                <option>Sri Lanka</option>
                <option>UK</option>
                <option>USA</option>
            </select>
        </div>
        <div class="element">
            <label>Email</label>
            <input class="Tinput" type="email" placeholder="Enter E-mail...">
        </div>
        <div class="element">
            <label>Phone</label>
            <input class="Tinput" type="number" placeholder="Enter Phone...">
        </div>
        <div class="element">
            <label>Password</label>
            <input class="Tinput" type="password">
        </div>
        <div class="element">
            <label>Confirm Password</label>
            <input class="Tinput" type="password">
        </div>
        <div class="element radio" style="padding-left: 20%">
            <input type="checkbox" style="margin-right: 4%;">
            <label>I agree to the Terms of Use</label>
        </div>

        <div class="buttons">
            <button class="button sub" type="submit">
                submit
            </button>
            <button class="button can" type="reset">
                Cancel
            </button>
        </div>

    </div>
    
</body>
</html>