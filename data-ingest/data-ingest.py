#!/usr/bin/env python
# coding: utf-8

import argparse
import os
import sys
from time import time
from typing import Any

import pandas as pd
from sqlalchemy import create_engine


# Create a parser to parse the command-line arguments
def create_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Ingest data into database", add_help=True, exit_on_error=True)

    parser.add_argument("--user", help="Username for the database", default="root", type=str)
    parser.add_argument("--password", help="Password for the database", default="root", type=str)
    parser.add_argument("--host", help="Host for the database", default=5432, type=int)
    parser.add_argument("--port", help="Port for the database", default=5432, type=int)
    parser.add_argument("--name", help="Name for the database", default="postgres", type=str)
    parser.add_argument(
        "--table-name", help="name of the table where we will write the results to", default="data", type=str
    )

    parser.add_argument("--url", help="URL of the csv data that will ingest", required=True, type=str)

    return parser


def main():
    # Parse arguments
    parser: argparse.ArgumentParser = create_parser()

    if len(sys.argv[1:]) == 0 or "--url" not in sys.argv[1:]:
        parser.print_help()
        parser.exit()
    else:
        args: Any = parser.parse_args()

    # Create engine
    engine = create_engine(f"postgresql://{args.user}:{args.password}@{args.host}:{args.port}/{args.name}")

    # Read data as in chunks
    df_iter = pd.read_csv(args.url, iterator=True, chunksize=100000)

    df = next(df_iter)

    df.tpep_pickup_datetime = pd.to_datetime(df.tpep_pickup_datetime)
    df.tpep_dropoff_datetime = pd.to_datetime(df.tpep_dropoff_datetime)

    # Getting column names
    df.head(n=0).to_sql(name=args.table_name, con=engine, if_exists="replace")

    # Inserting data into the database
    df.to_sql(name=args.table_name, con=engine, if_exists="append")

    while True:
        # Iterate over the chunks & insert into the database
        try:
            t_start = time()

            df = next(df_iter)

            df.tpep_pickup_datetime = pd.to_datetime(df.tpep_pickup_datetime)
            df.tpep_dropoff_datetime = pd.to_datetime(df.tpep_dropoff_datetime)

            df.to_sql(name=args.table_name, con=engine, if_exists="append")

            t_end = time()

            print("inserted another chunk, took %.3f second" % (t_end - t_start))

        except StopIteration:
            print("Finished ingesting data into the postgres database")
            break


if __name__ == "__main__":
    main()
