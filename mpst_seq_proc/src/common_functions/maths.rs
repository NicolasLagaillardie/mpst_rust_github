// use std::convert::TryFrom;

type VecOfTuple = Vec<(u64, u64, u64)>;

/// Create the whole matrix of index according to line and column
pub(crate) fn diag(number_roles: u64) -> VecOfTuple {
    let diff = number_roles - 1;

    let mut column = 0;
    let mut line = 0;

    // Create the upper diag
    (0..(diff * (diff + 1) / 2))
        .map(|i| {
            if line == column
            {
                column += 1;
            }
            else if column >= (number_roles - 1)
            {
                line += 1;
                column = line + 1;
            }
            else
            {
                column += 1;
            }
            (line + 1, column + 1, i + 1)
        })
        .collect()
}

/// Create the whole matrix of index according to line and column
pub(crate) fn diag_and_matrix(number_roles: u64) -> (Vec<VecOfTuple>, VecOfTuple) {
    let diag = diag(number_roles);

    // Create the whole matrix
    (
        (1..=number_roles)
            .map(|i| {
                diag.iter()
                    .filter_map(|(line, column, index)| {
                        if i == *line || i == *column
                        {
                            Some((*line, *column, *index))
                        }
                        else
                        {
                            None
                        }
                    })
                    .collect()
            })
            .collect(),
        diag,
    )
}

/// Return (line, column, index) of matrix
pub(crate) fn get_tuple_matrix(matrix: &[VecOfTuple], i: u64, j: u64) -> (u64, u64, u64) {
    let list: VecOfTuple = if let Some(temp) = matrix.get(usize::try_from(i - 1).unwrap())
    {
        temp.to_vec()
    }
    else
    {
        panic!(
            "Error at get_tuple_matrix for i = {:?} / matrix = {:?}",
            i, matrix
        )
    };

    if let Some((line, column, index)) = list.get(usize::try_from(j - 1).unwrap())
    {
        (*line, *column, *index)
    }
    else
    {
        panic!(
            "Error at get_tuple_matrix for i = {:?} and j = {:?} with list = {:?} / matrix = {:?}",
            i, j, list, matrix
        )
    }
}

/// Return (line, column) of diag from index
pub(crate) fn get_line_column_from_diag(diag: &[(u64, u64, u64)], index: u64) -> (u64, u64) {
    for i in diag
    {
        if i.2 == index
        {
            return (i.0, i.1);
        }
    }
    panic!("Error at get_line_column_from_diag for index = {:?}", index)
}

/// Create the whole matrix of index according to line and column
pub(crate) fn diag_w_offset(number_roles: u64) -> VecOfTuple {
    let diff = number_roles - 1;

    let mut column = 0;
    let mut line = 0;

    // Create the upper diag
    (0..=(diff * (diff + 1) / 2))
        .map(|i| {
            if line == column
            {
                column += 1;
            }
            else if column >= (number_roles - 1)
            {
                line += 1;
                column = line + 1;
            }
            else
            {
                column += 1;
            }
            (line + 1, column + 1, i + 1)
        })
        .collect()
}

/// Create the whole matrix of index according to line and column
pub(crate) fn diag_and_matrix_w_offset(number_roles: u64) -> (Vec<VecOfTuple>, VecOfTuple) {
    let diag_w_offset = diag_w_offset(number_roles);

    // Create the whole matrix
    (
        (1..=number_roles)
            .map(|i| {
                diag_w_offset
                    .iter()
                    .filter_map(|(line, column, index)| {
                        if i == *line || i == *column
                        {
                            Some((*line, *column, *index))
                        }
                        else
                        {
                            None
                        }
                    })
                    .collect()
            })
            .collect(),
        diag_w_offset,
    )
}

/// Return (line, column, index) of diag
pub(crate) fn get_tuple_diag(diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
    if let Some((line, column, index)) = diag.get(usize::try_from(i - 1).unwrap())
    {
        (*line, *column, *index)
    }
    else
    {
        panic!(
            "Error at get_tuple_diag for i = {:?} / diag = {:?}",
            i, diag
        )
    }
}
