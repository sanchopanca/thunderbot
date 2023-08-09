INSERT INTO rules (name, updated_by) VALUES
('kpop', 'admin'),
('what a week', 'admin'),
('table flip', 'admin'),
('boys talking', 'admin');

INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'kpop time', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'k p o p   t i m e', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'kpop tijd', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'hat a week, huh', id, 'admin' FROM rules WHERE name = 'what a week';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'hat a week huh', id, 'admin' FROM rules WHERE name = 'what a week';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT '(╯°□°)╯︵ ┻━┻', id, 'admin' FROM rules WHERE name = 'table flip';
INSERT INTO patterns (pattern, rule_id, updated_by)
SELECT 'bot, what are they talking about', id, 'admin' FROM rules WHERE name = 'boys talking';

INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/9bZkp7q19f0', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/POe9SOEKotk', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/5UdoUmvu_n8', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/2e-Q7GfCGbA', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/id6q2EP2UqE', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/8dJyRm2jJ-U', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/JQGRg8XBnB4', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/Hbb5GPxXF1w', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/p1bjnyDqI9k', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/k6jqx9kZgPM', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/z8Eu-HU0sWQ', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/eH8jn4W8Bqc', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/IHNzOHi8sJs', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/WPdWvnAAurg', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/gdZLi9oWNZg', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/H8kqPkEXP_E', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/awkkyBH2zEo', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/z3szNvgQxHo', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/i0p1bmr0EmE', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/WyiIGEHQP8o', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://youtu.be/lcRV2gyEfMo', id, 'admin' FROM rules WHERE name = 'kpop';
INSERT INTO responses (response, rule_id, updated_by)
SELECT 'https://whataweek.eu', id, 'admin' FROM rules WHERE name = 'what a week';
INSERT INTO responses (response, rule_id, updated_by)
SELECT '┬─┬ノ(º_ºノ)', id, 'admin' FROM rules WHERE name = 'table flip';
INSERT INTO responses (response, rule_id, updated_by)
SELECT "Summarization functionality has been turned off until the time it's possible to run a decent LLM on a cheap VPS", id, 'admin' FROM rules WHERE name = 'boys talking';


