#!/bin/sh

if [ -f "sample.db" ]; then 
   rm -f sample.db
fi
sqlite3 sample.db < sample_sqlite3.sql
