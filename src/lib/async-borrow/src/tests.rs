use super::*;
use static_cell::ConstStaticCell;
use tokio::time::{Duration, sleep, timeout};

/// Basic integration test on the happy path, with borrower and lender passed down to the parts.
///
/// In its three lending cases it illustrates different versions of dropping the borrowed value,
/// but also uses short sleeps to ensure that cases of "borrower waits for lender" and "lender
/// waits for borrower" are covered. (They currently don't take different code paths, but might
/// after optimization).
#[tokio::test]
async fn all_locally() {
    static EXCHANGE: ConstStaticCell<AsyncBorrowExchange<i128>> =
        ConstStaticCell::new(AsyncBorrowExchange::new());
    let (lender, borrower) = EXCHANGE.take().split();

    async fn lend_and_check_increment_3x(item: &mut i128, mut lender: Lender<i128>) {
        assert!(*item == 0);
        // First round: borrower is already active.
        sleep(Duration::from_millis(100)).await;
        lender.lend(item).await;
        assert!(*item == 1);
        // Second round: borrower is not yet ready, as it sleeps a bit after the first round.
        lender.lend(item).await;
        assert!(*item == 2);
        // Third round: nobody sleeps before, but borrower sleeps while it holds the borrow.
        lender.lend(item).await;
        assert!(*item == 3);
    }

    async fn borrow_and_increment_3x(mut borrower: Borrower<i128>) {
        let mut borrowed = borrower.borrow().await;
        *borrowed += 1;
        // Has to be done explicitly…
        drop(borrowed);

        sleep(Duration::from_millis(100)).await;

        {
            let mut borrowed = borrower.borrow().await;
            *borrowed += 1;
        }
        // … or through scope, like a lock before it can be acquired again.

        borrowed = borrower.borrow().await;
        sleep(Duration::from_millis(100)).await;
        *borrowed += 1;
    }

    let mut local = 0;
    tokio::join!(
        lend_and_check_increment_3x(&mut local, lender),
        borrow_and_increment_3x(borrower),
    );
}

#[tokio::test]
#[should_panic(
    expected = "Task that lends data must not be dropped, panicking in drop handler to avoid use-after-free."
)]
async fn lender_dropped() {
    static EXCHANGE: ConstStaticCell<AsyncBorrowExchange<i128>> =
        ConstStaticCell::new(AsyncBorrowExchange::new());
    let (lender, borrower) = EXCHANGE.take().split();

    async fn lend_and_check_increment(item: &mut i128, mut lender: Lender<i128>) {
        assert!(*item == 0);
        timeout(Duration::from_millis(100), async {
            lender.lend(item).await;
        })
        .await
        // We would expect the timeout to bite, but also know it won't come to that due to the drop
        // guard.
        .unwrap_err();
        assert!(*item == 1);
    }

    async fn borrow_and_increment(mut borrower: Borrower<i128>) {
        let mut borrowed = borrower.borrow().await;
        sleep(Duration::from_millis(200)).await;
        *borrowed += 1;
    }

    let mut local = 0;
    tokio::join!(
        lend_and_check_increment(&mut local, lender),
        borrow_and_increment(borrower),
    );
}
