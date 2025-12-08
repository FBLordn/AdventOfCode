@echo off
CD "2025"
MD "day%1"
CD "day%1"
type nul > day%1.py
type nul > day%1.txt
type nul > test.txt