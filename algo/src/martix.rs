use core::fmt;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread, vec,
};

use anyhow::anyhow;

use crate::ventor::{dot_product, Vector};

const NUM_THREADS: usize = 4;

pub struct Matrix<T: Display> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Display + Default + Send + 'static,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply_conc(&self, &rhs).expect("Matrix multiply error")
    }
}

impl<T: Display> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        let res = Self {
            data: data.into(),
            row,
            col,
        };
        assert_eq!(res.data.len(), row * col);
        res
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply_conc<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Display + Default + Send + 'static,
{
    if a.row != b.col {
        return Err(anyhow!("invalid: a.raw != b.col"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(|| {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

pub fn multiply_single<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Display + Default,
{
    if a.row != b.col {
        return Err(anyhow!("invalid: a.raw != b.col"));
    }

    let mut data = vec![T::default(); a.row * b.col];

    // a[i][j] = i * col + j
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]); // a.data[i]
            let col = Vector::new(
                b.data[j..]
                    .iter()
                    .step_by(b.col)
                    .copied()
                    .collect::<Vec<_>>(),
            );
            data[i * b.col + j] += dot_product(row, col)?
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[cfg(test)]
mod tests {
    use super::{multiply_conc, multiply_single, Matrix};

    #[test]
    fn test_multiply_single() {
        let m1 = Matrix::new(vec![1, 0, 2, -1, 3, 1], 2, 3);
        let m2 = Matrix::new(vec![3, 1, 2, 1, 1, 0], 3, 2);
        let res = multiply_single(&m1, &m2);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.col, 2);
        assert_eq!(res.row, 2);
        assert_eq!(res.data, vec![5, 1, 4, 2]);
    }

    #[test]
    fn test_multiply_conc() {
        let m1 = Matrix::new(vec![1, 0, 2, -1, 3, 1], 2, 3);
        let m2 = Matrix::new(vec![3, 1, 2, 1, 1, 0], 3, 2);
        let res = multiply_conc(&m1, &m2);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.col, 2);
        assert_eq!(res.row, 2);
        assert_eq!(res.data, vec![5, 1, 4, 2]);
    }

    #[test]
    fn test_multiply_conc_mul() {
        let m1 = Matrix::new(vec![1, 0, 2, -1, 3, 1], 2, 3);
        let m2 = Matrix::new(vec![3, 1, 2, 1, 1, 0], 3, 2);
        let res = m1 * m2;
        assert_eq!(res.col, 2);
        assert_eq!(res.row, 2);
        assert_eq!(res.data, vec![5, 1, 4, 2]);
    }

    #[test]
    #[should_panic]
    fn test_multiply_conc_mul_panic() {
        let m1 = Matrix::new(vec![1, 0, 2, -1, 3, 1], 2, 2);
        let m2 = Matrix::new(vec![3, 1, 2, 1, 1, 0], 3, 2);
        let _ = m1 * m2;
    }
}
