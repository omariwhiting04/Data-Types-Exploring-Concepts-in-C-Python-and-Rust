def grade_category(g):
    if g < 50:
        return "Fail"
    elif g < 70:
        return "Pass"
    elif g < 80:
        return "Credit"
    elif g < 90:
        return "Distinction"
    else:
        return "High Distinction"

grades = [45, 67, 72, 85, 93, 58, 79, 88, 100]
fail = pass_ = credit = dist = high = 0

for g in grades:
    cat = grade_category(g)
    print(g, "->", cat)
    if cat == "Fail": fail += 1
    elif cat == "Pass": pass_ += 1
    elif cat == "Credit": credit += 1
    elif cat == "Distinction": dist += 1
    else: high += 1

print(f"Summary: F={fail} P={pass_} C={credit} D={dist} HD={high}")
