"""
Test goes here

"""

import subprocess


def test_init():
    """tests extract()"""
    result = subprocess.run(
        ["python", "main.py", "init"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_update_record():
    """tests update_record"""
    result = subprocess.run(
        [
            "python",
            "main.py",
            "update_record",
            "4",
            "2.3859",
            "15.0",
            "3.8271604938271606",
            "1.112099644128114",
            "1280.0",
            "2.486988847583643",
            "34.6",
            "-120.12",
            "0.98",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_delete_record():
    """tests delete_record"""
    result = subprocess.run(
        ["python", "main.py", "delete_record", "--id", "1"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_create_record():
    """tests create_record"""
    result = subprocess.run(
        [
            "python",
            "main.py",
            "create_record",
            "2.3859",
            "15.0",
            "3.8271604938271606",
            "1.112099644128114",
            "1280.0",
            "2.486988847583643",
            "34.6",
            "-120.12",
            "0.98",
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


def test_read_data():
    """tests read_data"""
    result = subprocess.run(
        ["python", "main.py", "read"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":
    test_init()
    test_create_record()
    test_read_data()
    test_update_record()
    test_delete_record()
