#!/bin/bash

# Set up Git user name and email
read -p "Enter your Git user name: " git_user_name
git config --global user.name "$git_user_name"

read -p "Enter your Git user email: " git_user_email
git config --global user.email "$git_user_email"