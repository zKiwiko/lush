/// @desc Clamp a number between a minimum and maximum value.
/// @param value integer|number The number to clamp.
/// @param min integer|number The minimum value to clamp to.
/// @param max integer|number The maximum value to clamp to.
/// @return number Clamped_value The clamped value.
pub fn clamp((value, min, max): (f64, f64, f64)) -> mlua::Result<f64> {
    if min > max {
        return Ok(value.clamp(max, min));
    }
    Ok(value.clamp(min, max))
}

/// @desc Linearly interpolates between two values based on a parameter t.
/// @param value integer|number The starting value.
/// @param target integer|number The target value to interpolate towards.
/// @param t integer|number The interpolation factor (0.0 to 1.0). A value of 0.0 will return the starting value, while a value of 1.0 will return the target value.
/// @return number Interpolated_value The result of the linear interpolation between the starting value and the target value based on the interpolation factor t.
pub fn lerp((start, end, t): (f64, f64, f64)) -> mlua::Result<f64> {
    Ok(start * (1.0 - t) + end * t)
}

/// @desc Returns the sign of a number. The function accepts a single numeric argument and returns -1 if the number is negative, 1 if the number is positive, and 0 if the number is zero.
/// @param value integer|number The number for which to determine the sign. The function accepts any numeric value, including integers and floating-point numbers. The sign is determined based on whether the value is negative, positive, or zero.
/// @return number Sign The sign of the input number. The function returns -1 if the input value is negative, 1 if the input value is positive, and 0 if the input value is zero. The result is returned as a number (integer or floating-point) depending on the input type, but it will always be one of the three possible values: -1, 0, or 1.
pub fn sign(value: f64) -> mlua::Result<f64> {
    Ok(value.signum())
}

/// @desc Calculates the mean (average) of a table of numbers. The function accepts a single argument, which must be a table containing numeric values. It iterates through the values in the table, sums them up, and divides by the count of values to compute the mean. If the table is empty or contains non-numeric values, an error is returned.
/// @param values table A table containing numeric values for which to calculate the mean. The table can be an array-like table (with integer keys starting from 1) or a table with arbitrary keys, as long as the values are numeric (integers or numbers).
/// @return number Mean The calculated mean (average) of the numeric values in the table. The result is returned as a
pub fn mean(values: mlua::Table) -> mlua::Result<f64> {
    let mut sum = 0.0f64;
    let mut count = 0usize;

    for pair in values.pairs::<mlua::Value, f64>() {
        let (_, val) = pair?;
        sum += val;
        count += 1;
    }

    if count == 0 {
        return Err(mlua::Error::RuntimeError(
            "math.mean cannot compute the mean of an empty table".into(),
        ));
    }

    Ok(sum / count as f64)
}

/// @desc Calculates the median of a table of numbers. The function accepts a single argument, which must be a table containing numeric values. It collects the values from the table, sorts them, and then computes the median based on whether the count of values is odd or even. If the table is empty or contains non-numeric values, an error is returned.
/// @param values table A table containing numeric values for which to calculate the median. The table can be an array-like table (with integer keys starting from 1) or a table with arbitrary keys, as long as the values are numeric (integers or numbers).
/// @return number Median The calculated median of the numeric values in the table.
pub fn median(values: mlua::Table) -> mlua::Result<f64> {
    let mut nums = Vec::with_capacity(values.len()? as usize);

    for pair in values.pairs::<mlua::Value, f64>() {
        nums.push(pair?.1);
    }

    let len = nums.len();
    if len == 0 {
        return Err(mlua::Error::RuntimeError(
            "math.median cannot compute an empty table".into(),
        ));
    }

    let mid = len / 2;

    if len % 2 == 0 {
        nums.select_nth_unstable_by(mid, |a, b| a.total_cmp(b));
        let val1 = nums[mid];

        let (left, _, _) = nums.select_nth_unstable_by(mid, |a, b| a.total_cmp(b));
        let val2 = *left.iter().max_by(|a, b| a.total_cmp(b)).unwrap_or(&val1);

        Ok((val1 + val2) / 2.0)
    } else {
        nums.select_nth_unstable_by(mid, |a, b| a.total_cmp(b));
        Ok(nums[mid])
    }
}
