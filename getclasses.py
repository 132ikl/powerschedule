import re
from csv import DictWriter
from typing import Union
from dataclasses import dataclass, field, asdict

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import Select, WebDriverWait

from time import sleep


def format_semester(raw):
    season = raw.split(" ")[0]
    year = ""
    if "odd" in raw:
        year = "Odd"
    if "even" in raw:
        year = "Even"
    return f"{season}{year}"


@dataclass
class Course:
    subject: str
    number: int
    semesters: str
    credits: int
    requisites: str

    @classmethod
    def from_raw(cls, course, divs):
        semesters = "|".join(format_semester(sem) for sem in divs[0].text.split(", "))
        credits = int(re.match(r"Total Credits: (\d+)", divs[1].text)[1])
        requisites = divs[2].text
        return Course(course[:3], course[3:], semesters, credits, requisites)

    def __repr__(self):
        out = f"{self.subject} {self.number} (\n"
        out += f"\tSemesters: {self.semesters}\n"
        out += f"\tCredits: {self.credits}\n"
        for line in repr(self.requisites).splitlines():
            out += f"\t{line}\n"
        out += "\n)"
        return out


def get_course_data(driver, course):
    driver.get(url)
    subject = Select(driver.find_element(By.ID, "MainContent_ddlSubjectCode"))
    coursenum = driver.find_element(By.ID, "MainContent_txtCourseNumber")
    submit = driver.find_element(By.ID, "MainContent_btnSubmit")

    subject.select_by_value(course[:3])
    coursenum.send_keys(course[3:])
    submit.click()

    xpath = "//div[@id='MainContent_rptrSearchResults_divMainDetails_0']/div/div[contains(@class, 'col-md-7')]"
    divs = driver.find_elements(By.XPATH, xpath)
    return Course.from_raw(course, divs)


with open("input.txt") as file:
    raw_courses = file.read().splitlines()
    raw_courses = [line for line in raw_courses if not line.startswith("#")]


url = "https://reg.msu.edu/Courses/Search.aspx"
browser = webdriver.Firefox()
browser.implicitly_wait(10)
browser.get(url)

courses = [get_course_data(browser, course) for course in raw_courses]

with open("input.csv", "w") as csvfile:
    writer = DictWriter(
        csvfile,
        fieldnames=["subject", "number", "credits", "semesters", "requisites"],
    )
    writer.writeheader()
    for course in courses:
        writer.writerow(asdict(course))
