### Steps to Run Payments Engine
- Create a file called transactions.csv with input data in root folder.
- Run the payments engine by executing `cargo run -- transactions.csv`.
- Output will be displayed in csv format in the console.
- Execute `cargo run -- transactions.csv > accounts.csv` to output in to accounts.csv.

### Test Data
- Engine has implemented all scenarios such as deposit, withdrawal, dispute, resolve and chargeback.
- Engine does not store entire input file in the memory. Hence, it can handle large input files efficiently.
- Below are two example inputs with which you could test.

| type | client | tx | amount |
| --- | --- | --- | --- |
| deposit | 1 | 1 | 1.0 |
| withdrawal | 1 | 2 | 1.0 |
| deposit | 1 | 3 | 5.0 |
| dispute | 1 | 3 | |
| resolve | 1 | 3 | |

| type | client | tx | amount |
| --- | --- | --- | --- |
| deposit | 1 | 1 | 1.0 |
| withdrawal | 1 | 2 | 1.0 |
| deposit | 1 | 3 | 5.0 |
| dispute | 1 | 3 | |
| chargeback | 1 | 3 | |

### Assumptions
Below assumtions have been made:
- Whitespaces are possible in each cell of the CSV.
- Input file will be in valid CSV format. Otherwise, engine will panic.
- Data type in each cell of CSV will be valid. Otherwise, engine will panic.
- Only deposits can be disputed.
- No further transaction will be possible on locked accounts.
- It is possible that an account can end up with negative available/total amount.
- Below is an example of negative total amount.

| type | client | tx | amount |
| --- | --- | --- | --- |
| deposit | 1 | 1 | 1.0 |
| withdrawal | 1 | 2 | 1.0 |
| dispute | 1 | 1 | |
| chargeback | 1 | 1 | |