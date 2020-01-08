def latest(scores):
    return scores[-1]


def personal_best(scores):
    return max(scores)


def personal_top_three(scores):
    AMT_TO_RETRIEVE = 3
    scores.sort(reverse=True)
    return scores[:AMT_TO_RETRIEVE]
